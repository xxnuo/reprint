use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};
use std::thread;

use common::config;
use common::wyhash;

fn main() {
    let _c = exec_base("./target/debug/generator.exe");
    println!("{:?}", _c);
    // todo!(|| { exec("",false) });
}

/// 执行单条命令 - 基本功能
/// todo: 目前没有处理包含管道重定向符的情况，默认处理普通的一条命令
fn exec_base(command_str: &str) -> ExitCode {
    let _command_str = command_str.trim();
    if _command_str.len() == 0 {
        return ExitCode::FAILURE;
    }
    let mut pieces = _command_str.split_whitespace();
    let command_path = pieces.next().unwrap();

    // 测试用，查看具体参数
    // for i in pieces.clone().into_iter() {
    //     println!("{:?}", i);
    // }

    let mut command = Command::new(command_path)
        .args(pieces)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("无法运行指定程序");

    let p_stdout = command.stdout.take().expect("无法捕获标准输出");
    let p_stderr = command.stderr.take().expect("无法捕获标准错误");

    let reader_stdout = BufReader::new(p_stdout);
    let reader_stderr = BufReader::new(p_stderr);

    // 子线程传递并处理标准输出和标准错误
    let thread_stdout = thread::spawn(move || {
        reader_stdout
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| {
                let s = process_single_stdout_and_err(&line);
                if s.is_empty() {
                    println!("{}", line);
                } else {
                    println!("{}", s);
                }
            });
    });
    let thread_stderr = thread::spawn(move || {
        reader_stderr
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| {
                let s = process_single_stdout_and_err(&line);
                if s.is_empty() {
                    eprintln!("{}", line);
                } else {
                    eprintln!("{}", s);
                }
            });
    });

    thread_stdout.join().expect("无法加入线程:标准输出");
    thread_stderr.join().expect("无法加入线程:标准错误");

    // 获取退出码并传递给调用方
    let mut exitcode: u8 = 0;
    match command.try_wait() {
        Ok(Some(status)) => {
            exitcode = status.code().unwrap() as u8;
        }
        Ok(None) => {
            // println!("status not ready yet, let's really wait");
            let res = command.wait().unwrap();
            exitcode = res.code().unwrap() as u8;
        }
        Err(e) => eprintln!("等待子线程退出出错:{}", e),
    };

    ExitCode::from(exitcode)
}

fn process_single_stdout_and_err(line: &str) -> String {
    let _hash = wyhash::hash(line);
    let _file_name = _hash + ".txt";
    let _new_path = PathBuf::from(&config::PATHS.1).join(&_file_name);
    if _new_path.is_file() {
        let mut _new_file = match File::open(&_new_path) {
            Err(e) => panic!("{:?}:无法读入Patch文件{}的内容", e, _new_path.display()),
            Ok(file) => file,
        };
        let mut s = String::new();
        match _new_file.read_to_string(&mut s) {
            Err(e) => panic!("{:?}:无法读入Patch文件{}内容到文本", e, _new_path.display()),
            Ok(_) => (),
        };
        return s;
    }

    let _raw_path = PathBuf::from(&config::PATHS.0).join(&_file_name);
    if !_raw_path.is_file() {
        let mut _raw_file = match File::create(&_raw_path) {
            Err(e) => panic!("无法创建输出文件{}:{:?}", _raw_path.display(), e),
            Ok(file) => file,
        };
        match _raw_file.write(line.as_bytes()) {
            Err(e) => panic!("无法写入输出文件{}:{:?}", _raw_path.display(), e),
            Ok(_) => (),
        }
    }
    return String::new();
}

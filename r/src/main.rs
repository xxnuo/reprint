use std::io::{BufRead, BufReader};
use std::process::{Command, ExitCode, Stdio};
use std::thread;

use common::config;

fn main() {
    let (_raw_path, _new_path) = config::config_init();
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
            .for_each(|line| println!("> {}", line));
    });
    let thread_stderr = thread::spawn(move || {
        reader_stderr
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| eprintln!("{}", line));
    });

    thread_stdout.join().expect("无法加入线程：标准输出");
    thread_stderr.join().expect("无法加入线程：标准错误");

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
        Err(e) => eprintln!("等待子线程退出出错：{}", e),
    };

    ExitCode::from(exitcode)
}

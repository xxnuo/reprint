use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{Child, Command, ExitCode, Stdio};
use std::{env, panic, thread};

use common::config;
use common::wyhash;

/// 下面的是调用内置命令例子，位置在data/tools/clean.exe，文件来自MSYS2
/// 目前仅仅调用exe，相比其他方法的优势可能就是不在PATH里显示，然并卵，并非真正的内置命令
/// TODO: 编写真正的builtin命令模块
const BUILTIN_COMMAND: [&str; 1] = ["clean"];

fn main() -> ExitCode {
    // 为release模式提供更友好的错误提示
    if !cfg!(debug_assertions) {
        // println!("debug");
        panic::set_hook(Box::new(|info| {
            if let Some(s) = info.payload().downcast_ref::<String>() {
                eprintln!("{}", s);
            }
        }));
    }
    let mut args = env::args();
    args.next(); // r.exe path
    let _command = match args.next() {
        Some(some) => {
            //添加环境变量，调用当前目录的文件或exe不需要添加./或.\
            config::add_path_efficient();
            some
        }
        // 没有命令输入，直接退出
        None => return ExitCode::SUCCESS,
    };
    let _command_str = _command.trim();

    // 内置命令
    if BUILTIN_COMMAND.contains(&_command_str) {
        return exec_raw(
            PathBuf::from(&config::PATHS.2)
                .join("tools")
                .join(format!("{}.exe", _command_str))
                .to_str()
                .expect("路径转换出错"),
            args,
        );
    }
    if _command_str == "clear" || _command_str == "clean" {
        return exec_raw(
            PathBuf::from(&config::PATHS.2)
                .join("tools")
                .join("clear.exe")
                .to_str()
                .expect("路径转换出错"),
            args,
        );
    }
    // 读取不执行的命令的名单，防止出现未知错误
    let mut _blocklist: Vec<String> = Vec::new();
    let _blocklist_path = PathBuf::from(&config::PATHS.2).join("blocklist.txt");
    if _blocklist_path.is_file() {
        let _blocklist_file =
            File::open(_blocklist_path).expect("白名单文件读取错误，如不使用请删除白名单文件");
        for line in BufReader::new(_blocklist_file).lines() {
            match line {
                Ok(some) => _blocklist.push(some),
                Err(_) => continue,
            }
        }
    }
    // 匹配
    for name in _blocklist {
        if _command_str == name.trim() {
            // println!("blocked!");
            return exec_raw(_command_str, args);
        }
    }

    // 开始执行
    let _rec = exec_base(_command_str, args);
    // println!("{:?}", &_rec);
    _rec
}
/// 执行单条命令 - 原样执行
fn exec_raw<I, S>(command_path: &str, pieces: I) -> ExitCode
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let command = Command::new(command_path)
        .args(pieces)
        .spawn()
        .expect("无法运行指定程序");

    // 获取退出码并传递给调用方
    get_exitcode(command)
}
/// 执行单条命令 - 带处理功能
fn exec_base<I, S>(command_path: &str, pieces: I) -> ExitCode
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
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
    get_exitcode(command)
}

fn get_exitcode(mut command: Child) -> ExitCode {
    // 获取退出码并传递给调用方
    let mut exitcode: u8 = 0;
    match command.try_wait() {
        Ok(Some(status)) => {
            exitcode = status.code().expect("无法获取程序退出码") as u8;
        }
        Ok(None) => {
            // println!("status not ready yet, let's really wait");
            let res = command.wait().expect("无法等待程序退出");
            exitcode = res.code().expect("无法获取程序退出码") as u8;
        }
        Err(e) => eprintln!("等待子线程退出出错:{}", e),
    };

    ExitCode::from(exitcode)
}

fn process_single_stdout_and_err(line: &str) -> String {
    if line.is_empty() {
        return String::new();
    }
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

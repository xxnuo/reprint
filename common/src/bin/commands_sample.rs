use std::io::{BufRead, BufReader};
use std::process::{Command, ExitCode, Stdio};
use std::thread;

fn main() {
    let _c = exec("./target/debug/generator.exe && uname", false);
    // println!("{:?}", _c);
    // todo!(|| { exec("",false) });
}

/// 执行使用 '|' '&' '&&' 连接的多条命令 - 提供给用户输入使用
/// uncheck: false - 由用户输入的内容，会对传入的command_str_full进行校验处理
/// uncheck: true - 由程序内部调用的命令，不需要trim检查
fn exec(command_str_full: &str, uncheck: bool) -> Vec<ExitCode> {
    let mut exit_codes: Vec<ExitCode> = Vec::new();
    let command_strs = command_str_full.trim();
    // 连接符有
    // &（顺序执行）、&&（前面成功执行后面）、||（前面失败执行后面）
    // 管道符有
    // |（输入到stdin）、>（输入到新文件）、>>（输入追加到文件）
    command_strs.split(&['|','']);

    // 执行单条命令并返回返回码
    if uncheck {
        exit_codes.push(exec_base(command_str));
    } else {
        exit_codes.push(exec_user(command_str));
    }

    exit_codes
}

/// 执行单条命令 - 提供给用户输入使用
/// 会对传入的command_str进行trim
fn exec_user(command_str: &str) -> ExitCode {
    let str = command_str.trim();
    if str.len() == 0 {
        return ExitCode::from(0);
    }
    exec_base(str)
}

/// 执行单条命令 - 基本功能
/// 不会对传入的command_str进行trim处理，需要自行处理。
/// todo: 目前没有处理包含管道重定向符的情况，默认处理普通的一条命令
fn exec_base(command_str: &str) -> ExitCode {
    let mut pieces = command_str.split_whitespace();
    let command_path = pieces.next().unwrap();
    let args = pieces;
    let mut command = Command::new(command_path)
        .args(args)
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

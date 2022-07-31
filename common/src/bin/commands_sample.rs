use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};
use std::thread;

fn main() -> Result<(), Error> {
    let mut command = Command::new("./target/debug/generator.exe")
        .args(["help", "error", "sleep", "error", "input"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("无法运行指定程序");

    let p_stdout = command
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "无法捕获标准输出"))?;
    let p_stderr = command
        .stderr
        .ok_or_else(|| Error::new(ErrorKind::Other, "无法捕获标准错误"))?;

    let reader_stdout = BufReader::new(p_stdout);
    let reader_stderr = BufReader::new(p_stderr);

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
            .for_each(|line| println!("> {}", line));
    });

    thread_stdout.join().unwrap();
    thread_stderr.join().unwrap();

    Ok(())
}

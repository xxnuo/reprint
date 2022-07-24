use std::process::Command;

fn main() {
    let output = Command::new("rustc --version")
        .arg("")
        .output()
        .unwrap_or_else(|e| panic!("执行进程出错：{}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("e:{}", s);
    }
}

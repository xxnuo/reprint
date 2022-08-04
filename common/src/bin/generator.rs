use std::env;
// use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut args = env::args();
    if args.len() > 1 {
        args.next();
        for arg in args {
            match arg.trim() {
                "sleep" => {
                    for i in 1..=3 {
                        println!("sleep -> {} output",i);
                        sleep(Duration::from_secs(1));
                    }
                    println!("end sleep");
                }
                "large" => {
                    println!("\n");
                }
                "error" => {
                    eprintln!("Make an error!");
                    // 测试不需要退出
                    // std::process::exit(138);
                }
                "input" => {
                    println!("Input 你好");
                    let mut line = String::new();
                    let bl = std::io::stdin().read_line(&mut line).unwrap();
                    println!("you input:{}",line.trim());
                    println!("字节数：{}",bl);
                }
                "help" => {
                    println!("sleep/ ?");
                }
                _ => continue,
            };
        }
    } else {
        println!("This is a sentence generator,\nType --help to see help.\nVersion: v1.2.3");
    }
}

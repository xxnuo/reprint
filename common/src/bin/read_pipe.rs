use std::io::{self, BufRead, Read};

fn main() {
    let _stdin = io::stdin();
    for line in _stdin.lock().bytes() {
        let line = line.expect("ERR");
        println!("> {}",line.to_string());
    }
}

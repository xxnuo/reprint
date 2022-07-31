use core::hash::Hasher;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{env, io};

use wyhash::wyhash;
use wyhash::WyHash;

fn main() {
    let mut args = env::args();
    args.next();
    for filename in args {
        if Path::new(&filename).is_file() {
            let result = hash_read(&filename);
            let msg = match result {
                Ok(r) => format!("{:x}",r),
                Err(e) => e.to_string(),
            };
            print!("{} *{}\n", msg, filename);
        } else {
            // 非文件，纯文本
            print!("{} *\"{}\"\n", hash_str(&filename), &filename);
        }
    }
}

fn hash_read(filename: &String) -> io::Result<u64> {
    read(filename, |r| hash(r))
}

fn read<F, T>(filename: &String, mut read_fn: F) -> io::Result<T>
where
    F: FnMut(&mut dyn BufRead) -> io::Result<T>,
{
    let mut reader = {
        let file = File::open(filename).map(BufReader::new)?;
        Box::new(file) as Box<dyn BufRead>
    };
    read_fn(&mut reader)
}

fn hash<R: BufRead>(mut reader: R) -> io::Result<u64> {
    let mut hasher = WyHash::with_seed(0);
    loop {
        let nb_bytes_read = {
            let bytes = reader.fill_buf()?;
            // EOF -> compute
            if bytes.is_empty() {
                return Ok(hasher.finish());
            }
            hasher.write(bytes);
            bytes.len()
        };
        reader.consume(nb_bytes_read);
    }
}

fn hash_str(content: &str) -> String {
    format!("{:x}", wyhash(content.as_bytes(), 0))
}

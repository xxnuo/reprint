use core::hash::Hasher;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::{env, io};
use wyhash::WyHash;

fn main() {
    let mut args = env::args();
    args.next();
    for (index, filename) in args.enumerate() {
        let now = Instant::now();
        let result = read_hash(&filename);
        let elapsed_time = now.elapsed();
        let elapsed_time_sec =
            elapsed_time.as_secs() as f64 + elapsed_time.subsec_nanos() as f64 * 1e-9;
        let msg = match result {
            Ok(r) => r,
            Err(e) => e.to_string(),
        };
        println!("{}", msg);
        println!(
            "\nFinish task {}'s BufRead() and Hash(0) took {} s.",
            index, elapsed_time_sec
        );
    }
}

fn read_hash(filename: &String) -> io::Result<String> {
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

fn hash<R: BufRead>(mut reader: R) -> io::Result<String> {
    let mut hasher = WyHash::with_seed(0);
    loop {
        let nb_bytes_read = {
            let bytes = reader.fill_buf()?;
            // EOF -> compute
            if bytes.is_empty() {
                return Ok(hasher.finish().to_string());
            }
            hasher.write(bytes);
            bytes.len()
        };
        reader.consume(nb_bytes_read);
    }
}

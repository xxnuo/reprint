use std::env;
use std::fs::create_dir_all;
use std::path::PathBuf;

const PROJ_ROOT_PATH: &str = r"C:\Users\bigtear\Documents\GitHub\reprint";

use lazy_static::lazy_static;
lazy_static! {
    ///PATHS 0：raw 文件夹；1：new 文件夹
    static ref PATHS: (PathBuf, PathBuf) = config_init();
}

fn main() {
    println!("{:?}", PATHS.0);
}

/// 获取输出目录和读取目录
pub fn config_init() -> (PathBuf, PathBuf) {
    let path = if cfg!(debug_assertions) {
        // debug 指定根目录
        PathBuf::from(PROJ_ROOT_PATH)
    } else {
        // release 模式的 data 目录在 exe 同级目录
        let mut _path = env::current_exe().expect("致命错误：无法获取可执行文件路径");
        _path.pop();
        _path
    };
    let path = path.join("data");
    // 获取到了 data 路径
    let raw_path = path.join("raw");
    let new_path = path.join("new");
    if !raw_path.is_dir() {
        create_dir_all(&raw_path).expect("致命错误：无法创建输出文件目录");
    };
    if !new_path.is_dir() {
        create_dir_all(&new_path).expect("致命错误：无法创建替换文件目录");
    };
    (raw_path, new_path)
}

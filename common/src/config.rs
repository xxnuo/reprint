use std::env;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::ffi::OsString;

const PROJ_ROOT_PATH: &str = r"C:\Users\bigtear\Documents\GitHub\reprint";

use lazy_static::lazy_static;
lazy_static! {
    ///PATHS 0:raw文件夹;1:new文件夹;2:data目录
    pub static ref PATHS: (PathBuf, PathBuf,PathBuf) = config_init();
}

// fn main() {
//     // println!("{}",PathBuf::from(r"C:\no-exist").join("sub").display());
//     let (_raw_path, _new_path) = config_init();
//     println!("{:?}", _raw_path);
// }

/// 获取输出目录和读取目录
pub fn config_init() -> (PathBuf, PathBuf, PathBuf) {
    let path = if cfg!(debug_assertions) {
        // debug 指定根目录
        PathBuf::from(PROJ_ROOT_PATH)
    } else {
        // release 模式的 data 目录在 exe 同级目录
        let mut _path = env::current_exe().expect("致命错误:无法获取可执行文件路径");
        _path.pop();
        _path
    };
    let path = path.join("data");
    // 获取到了 data 路径
    let raw_path = path.join("raw");
    let new_path = path.join("new");
    if !raw_path.is_dir() {
        create_dir_all(&raw_path).expect("致命错误:无法创建输出文件目录");
    };
    if !new_path.is_dir() {
        create_dir_all(&new_path).expect("致命错误:无法创建替换文件目录");
    };
    (raw_path, new_path, path)
}


/// 将运行目录添加到环境变量内
pub fn add_path_efficient() {
    //纯文本方式处理
    #[cfg(target_os = "windows")]
    const PATH_SEPARATOR: &str = ";";

    #[cfg(not(target_os = "windows"))]
    const PATH_SEPARATOR: &str = ":";

    // println!("{:#?}", PATH_SEPARATOR);

    let new_path = match env::var_os("PATH") {
        Some(path) => {
            let current_dir = env::current_dir().unwrap().into_os_string();
            let mut new_path = OsString::with_capacity(path.len() + current_dir.len() + 1);
            new_path.push(current_dir);
            new_path.push(OsString::from(PATH_SEPARATOR));
            new_path.push(path);
            new_path
        }
        _ => {
            eprintln!("无法获取PATH环境变量");
            OsString::new()
        }
    };
    env::set_var("PATH", &new_path);
    // println!("{:#?}", env::var_os("PATH"));
}

pub fn _add_path_traditional() {
    if let Some(path) = env::var_os("PATH") {
        let mut paths: Vec<PathBuf> = vec![env::current_dir().unwrap()];
        paths.extend(env::split_paths(&path));
        let new_path = env::join_paths(paths).unwrap();
        env::set_var("PATH", &new_path);
    }
    println!("{:#?}", env::var_os("PATH"));
}

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

fn main() {
    _add_path_efficient();
}

fn _add_path_efficient() {
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
        None => {
            eprintln!("无法获取PATH环境变量");
            OsString::new()
        }
    };
    println!("{:#?}", new_path);
}
fn _add_path_traditional() {
    if let Some(path) = env::var_os("PATH") {
        let mut paths: Vec<PathBuf> = vec![env::current_dir().unwrap()];
        paths.extend(env::split_paths(&path));
        let new_path = env::join_paths(paths).unwrap();
        env::set_var("PATH", &new_path);
    }
    println!("{:#?}", env::var_os("PATH"));
}

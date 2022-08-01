// add Cargo.toml
///
/// build = "build.rs"
//
// [target.'cfg(windows)'.build-dependencies]
// winres = "0.1.12"
// #winapi = "0.3.9"
//
// [package.metadata.winres]
// OriginalFilename = "r.exe"
// FileDescription = "命令行中间件"
// LegalCopyright = "Copyright © 2022"
// ProductVersion = "0.1.0"

#[cfg(windows)]
use winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_toolkit_path(r"C:\\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64");
    res.set_language(0x04);
    res.set_version_info()
    res.set_icon(r"res\icon.ico");
    res.set_output_directory(r".\");
    res.compile().unwrap();
}

#[cfg(unix)]
fn main() {
}
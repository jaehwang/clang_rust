//use std::env;
//use std::path::PathBuf;

fn main() {
    // Clang 라이브러리 경로
    let lib_path = if cfg!(target_os = "macos") {
        "/Applications/Xcode.app/Contents/Frameworks"
    } else if cfg!(target_os = "linux") {
        "/home/linuxbrew/.linuxbrew/Cellar/llvm/19.1.3/lib"
    } else {
        panic!("Unsupported OS");
    };

    // rpath 설정
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path);
}

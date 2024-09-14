//use std::env;
//use std::path::PathBuf;

fn main() {
    // Clang 라이브러리 경로 설정
    let lib_path = "/Applications/Xcode.app/Contents/Frameworks"; // Clang 라이브러리 경로

    // rpath 설정
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path);
}

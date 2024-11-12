# Clang을 이용한 호출 그래프 생성

Clang을 이용하여 C/C++ 코드의 호출 그래프를 생성하는 프로그램

## Homebrew를 쓰는 Linux에서 빌드 할 때

libclang.so를 설치해야 한다.
```sh
brew install llvm
```

clang_sys를 빌드 할 때 library 위치를 알려 줘야 한다.
```sh
export LIBCLANG_PATH=/home/linuxbrew/.linuxbrew/Cellar/llvm/19.1.3/lib
```

`build.rs`에서 `rustc-link-arg=-Wl,-rpath`를 지정해야 한다.

<!--
vim:nospell
-->

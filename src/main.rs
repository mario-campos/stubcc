use std::env;
use std::fs;
use std::path::Path;
use std::process;

const GCC_VERSION: &str = "gcc (Ubuntu 9.4.0-1ubuntu1~20.04.1) 9.4.0\nCopyright (C) 2019 Free Software Foundation, Inc.\nThis is free software; see the source for copying conditions.  There is NO\nwarranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.";
const CLANG_VERSION: &str = "Ubuntu clang version 11.0.0-2~ubuntu20.04.1\nTarget: x86_64-pc-linux-gnu\nThread model: posix\nInstalledDir: /usr/bin";
const STUBCC_VERSION: &str = concat!("stubcc", " ", env!("CARGO_PKG_VERSION"));

fn main() {
    // The first argument is, conventionally, the executable name.
    // Since stubcc can mimic gcc and clang, this first argument
    // will tell stubcc whether to mimic gcc or clang.
    let program = env::args().next().unwrap();
    let program_name = Path::new(&program).file_name().unwrap();

    for arg in env::args().skip(1) {
        if arg == "--version" {
            println!("{}", match program_name.to_str().unwrap() {
                "gcc" => GCC_VERSION,
                "clang" => CLANG_VERSION,
                _ => STUBCC_VERSION,
            });
            process::exit(0);
        }
    }

    for pair in env::args().zip(env::args().skip(1)) {
        let (flag, arg) = pair;
        if flag == "-o" {
            fs::write(arg, b"\x01S\x23T\x45U\x67B\x89C\xABC\xDE\xFF");
            process::exit(0);
        }
    }
}

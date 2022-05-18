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

    let mut output_mode = false;
    let mut compile_mode = false;

    for arg in env::args().skip(1) {
        if arg == "--version" {
            println!("{}", match program_name.to_str().unwrap() {
                "gcc" => GCC_VERSION,
                "clang" => CLANG_VERSION,
                _ => STUBCC_VERSION,
            });
            process::exit(0);
        }

        if arg == "-o" {
            output_mode = true;
        }

        if arg == "-c" {
            compile_mode = true;
        }
    }

    if output_mode {
        for pair in env::args().zip(env::args().skip(1)) {
            let (flag, arg) = pair;
            if flag == "-o" {
                // As a quick test of the compiler, the extractor will have the compiler compile a small snippet.
                // The extractor then looks for this particular byte sequence in the output file:
                //   0x01 0x53 0x23 0x45 0x45 0x4d 0x67 0x4d 0x89 0x4c 0xab 0x45 0xcd 0x21 0xef
                fs::write(arg, b"\x01S\x23E\x45M\x67M\x89L\xabE\xcd!\xef");
                process::exit(0);
            }
        }
    } else if compile_mode {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

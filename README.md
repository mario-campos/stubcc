# stubcc

stubcc is a fake (or stub) C compiler that mimics `gcc` or `clang` at a _very_ surface level. There is only one purpose for stubcc: to fool CodeQL into analyzing C/C++ code which would not be compiled by a real compiler.

#### Why?

I'm glad you asked :) stubcc was written so that CodeQL could analyze the many open-source BSD projects, which would not be analyzable unless they were already portable to a CodeQL-supported platform (Linux, macOS, Windows). Many of these BSD projects are not portable, but are still worthy of benefitting from some good ol' static analysis.

#### Do I need it?

No, probably not.

#### How do I install it?

1. Build stubcc using a recent version of the Rust toolchain:

```shell
cargo build --release
```

2. Install it as `gcc` or `clang`:

```shell
install -m 755 -o root -g root stubcc/target/release/stubcc /usr/local/bin/gcc
install -m 755 -o root -g root stubcc/target/release/stubcc /usr/local/bin/clang
```

3. Reference stubcc as your C compiler (remember: only when running CodeQL!):

```shell
make CC=/usr/local/bin/clang
```

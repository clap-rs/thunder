# ⚡ Thunder ⚡

[![](https://img.shields.io/travis/spacekookie/thunder.svg)](https://travis-ci.org/spacekookie/thunder/)
[![Build status](https://ci.appveyor.com/api/projects/status/clrwni1vork68vq6?svg=true)](https://ci.appveyor.com/project/spacekookie/thunder)
[![](https://img.shields.io/crates/v/thunder.svg)](https://crates.io/crates/thunder)
[![Docs.rs](https://docs.rs/thunder/badge.svg)](https://docs.rs/thunder/)

Write simple commandline applications in  Rust with *zero* boilerplate. This crate was inspired by [thor](https://github.com/erikhuda/thor) and uses [clap](https://github.com/kbknapp/clap-rs).

Experimental stage; **use at your own risk!**

## Example

```rust
// ... ignore the imports for now ...

struct MyApp;

/// Describe your application with style ✨
#[thunderclap]
impl MyApp {
    /// Say hello to someone
    fn hello(name: String) {
        println!("Hello {}", name);
    }
}

fn main() {
    MyApp::start();
}
```

**This prints**

```
USAGE:
    MyApp [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    hello    Say hello to someone
    help     Prints this message or the help of the given subcommand(s)
```

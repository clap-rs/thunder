[![](https://img.shields.io/travis/spacekookie/thunder.svg)](https://travis-ci.org/spacekookie/thunder/)
[![Build status](https://ci.appveyor.com/api/projects/status/clrwni1vork68vq6?svg=true)](https://ci.appveyor.com/project/spacekookie/thunder)
[![](https://img.shields.io/crates/v/thunder.svg)](https://crates.io/crates/thunder)
[![Docs.rs](https://docs.rs/thunder/badge.svg)](https://docs.rs/thunder/)

<br/>
<p align="center">
<img src="logo.svg" />
</p>
<br/>

Write simple commandline applications in Rust with *zero* boilerplate. Bind Rust functions to CLI functions and options with macros. This crate uses [clap.rs](https://github.com/kbknapp/clap-rs) for the actual argument parsing.

## Example

```rust,norun
// ... ignore the imports for now ...

struct MyApp;

/// Describe your application with style ✨
#[thunderclap]
impl MyApp {
    /// Say hello to someone
    fn hello(name: &str) {
        println!("Hello {}", name);
    }
}

fn main() {
    MyApp::start();
}
```

**This prints**

```norun
USAGE:
    MyApp [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    hello    Say hello to someone
    help     Prints this message or the help of the given subcommand(s)
```

Check the documentation for more examples.

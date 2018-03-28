# ⚡ Thunder ⚡

Write simple commandline applications in  Rust with *zero* boilerplate.


## Example

```rust
// ... ignore the imports for now ...

struct MyApp;

#[thunderclap]
impl MyApp {
    /// Say hello to someone
    fn hello(name: String) {
        println!("Hello {}", name);
    }

    fn bye() {
        println!("Bye!");
    }
}

fn main() {
    MyApp::start();
}
```

This prints

```
USAGE:
    MyApp [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    hello    <name>    
    help     Prints this message or the help of the given subcommand(s)⏎                              

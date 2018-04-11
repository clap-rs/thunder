//! Thor is the god of thunder
#![feature(proc_macro)]
extern crate clap;
extern crate thunder;
use thunder::thunderclap;

struct Thor;

/// An application that shoots lightning out of its hands
#[thunderclap]
impl Thor {
    /// Say hello to someone at home
    fn hello(name: &str) {
        println!("Hello {}", name);
    }

    /// Print bar
    fn foo() {
        println!("bar");
    }
}

fn main() {
    Thor::start();
}

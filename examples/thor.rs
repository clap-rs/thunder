//! Thor is the god of thunder

#![feature(proc_macro)]

extern crate clap;
extern crate thunder;
use thunder::thunderclap;

struct MyApp;

#[thunderclap]
impl MyApp {
    /// Say hello
    fn hello(name: String) {
        println!("Hello World!");
    }
}

fn main() {
    MyApp::start();
}

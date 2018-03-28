//! Thor is the god of thunder
#![feature(proc_macro)]
extern crate clap;
extern crate thunder;
use thunder::thunderclap;


struct Thor;

#[thunderclap]
/// An application that shoots lightning out of it's hands
impl Thor {

    /// Say hello to someone at home
    fn hello(name: String) {
        println!("Hello {}", name);
    }
}

fn main() {
    Thor::start();
}
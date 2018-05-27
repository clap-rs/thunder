#![feature(proc_macro)]
extern crate thunder;
extern crate clap;

use thunder::thunderclap;

struct Loki;

#[thunderclap(example: Option<String>: "Error, here", another_example: String: "No error, here")]
impl Loki {
    fn hello() {
        println!("{:?}", Self::example());
    }
}

fn main() {
    Loki::start();
}

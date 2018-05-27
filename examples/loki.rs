#![feature(proc_macro)]
extern crate thunder;
extern crate clap;

use thunder::thunderclap;

struct Loki;

#[thunderclap(example: Option<String>: "Error here")]
impl Loki {
    fn hello() {
        println!("Hello, {:?}!", Self::example());
    }
}

fn main() {
    Loki::start();
}

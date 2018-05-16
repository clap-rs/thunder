//! Thor is the god of thunder
#![feature(proc_macro)]
extern crate clap;
extern crate thunder;
use thunder::thunderclap;

const DEF: &'static str = "Hello to the world";

struct Thor;

/// An application that shoots lightning out of its hands
#[thunderclap(drunk: bool: "Thor drinks a lot", hammers: Option<u8>: "This isn't a joke about being drunk")]
impl Thor {

    /// Say hello to someone at home
    fn hello(name: &str) {
        println!("Hello {}", name);
    }

    /// Say goodbye. Or don't, if you're shy
    fn bye(name: Option<&str>) {
        println!("Not saying bye is rude: {:?}", name);
    }

    /// Thor will rudely comment on your age
    fn aged(age: Option<i128>) {
        println!("Ha, look at you being: {:?}", age);
    }

    /// Prints 'bar'
    fn foo() {
        println!("bar");
    }
}

fn main() {
    Thor::start();
}

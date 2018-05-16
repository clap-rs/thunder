//! Thor is the god of thunder
#![feature(proc_macro)]
extern crate clap;
extern crate thunder;
use thunder::thunderclap;

struct Thor;

/// An application that shoots lightning out of its hands
#[thunderclap(drunk: bool: "Thor drinks a lot", hammers: Option<u8>: "This isn't a joke about being drunk")]
impl Thor {
    /// Say hello to someone at home
    fn hello(name: &str) {
        println!(
            "Hello {}, your son Thor is {}",
            name,
            match Self::drunk() {
                true => "drunk",
                false => "not drunk!",
            }
        );

        println!(
            "Thor has {} hammers",
            match Self::hammers() {
                Some(n) => n,
                _ => 0,
            }
        );
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

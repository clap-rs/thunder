#![feature(proc_macro)] 

extern crate clap;
extern crate thunder;
use thunder::thunderclap;

struct MyApp;

#[thunderclap]
impl MyApp {
    fn hello() {
        println!("Hello World!");
    }

/*
    fn setup() -> App {
        return App::new("MyApp").subcommand(
            SubCommand::with_name("hello").arg(Arg::with_name("name")
            // ...
        );
    }
*/
}


fn main() {
    MyApp::hello();
    MyApp::start();

    // let app = MyApp::setup();
}
use std::{env, fs};

mod utils;
mod lexer;
mod parser;
mod vm;

use vm::{SContext, execute_string};

fn main() {
    let args : Vec<String> = env::args().collect();
    let code = "{".to_string() + &*fs::read_to_string(&args[1]).unwrap() + "}";

    let res = {
        let mut ctx = SContext::new();
        execute_string(&code, &mut ctx)
    };
    println!("{:?}", res);
}

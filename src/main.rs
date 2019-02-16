use std::fs;
use std::error::Error;
use std::process;
use std::env;
use std::io;

use chrono::prelude::*;

pub mod resolve;

fn main() {
    let conf = resolve::parser::Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem reading arguments: {}", err);
        process::exit(1);
    });
    resolve::parser::parser(conf);
//    for path in fs::read_dir("./").unwrap() {
//        println!("{}", path.unwrap().path().to_str().unwrap());
//    }
//    for path in fs::read_dir("./").unwrap() {
//        println!("{}", path.unwrap().file_type().unwrap().is_file());
//    }

}


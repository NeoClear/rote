use std::fs;
use std::error::Error;
use std::process;

use chrono::prelude::*;

mod parser;

fn main() {
    let conf = parser::Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem reading arguments: {}", err);
        process::exit(1);
    });
    parser::parser(conf);
}


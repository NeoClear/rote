use std::fs;
use std::error::Error;
use std::process;

use chrono::prelude::*;

pub mod resolve;

fn main() {
    let conf = resolve::parser::Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem reading arguments: {}", err);
        process::exit(1);
    });
    resolve::parser::parser(conf);
}


use std::fs;
use std::error::Error;
use std::process;

use chrono::prelude::*;

pub struct Config {
    pub command: String,
    pub sub_command: Option<String>,
    pub filename: Option<String>,
    pub description: Option<String>,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        let mut command: String;
        let mut sub_command: Option<String> = None;
        let mut filename: Option<String> = None;
        let mut description: Option<String> = None;

        args.next();
        let command = match args.next() {
            Some(args) => args,
            None => return Err("Didn't get command"),
        };

        match command {
            _ if command == "print" => {
                filename = match args.next() {
                    Some(args) => Some(args),
                    None => return Err("Didn't get filename"),
                };
            },
            _ if command == "gist" => {
                filename = match args.next() {
                    Some(args) => Some(args),
                    None => return Err("Didn't get filename"),
                };
            },
            _ if command == "gen" => {
                filename = match args.next() {
                    Some(args) => Some(args),
                    None => return Err("Didn't get filename"),
                };
            },
            _ => return Err("Command not found"),
        }

        Ok(Config {
            command,
            sub_command,
            filename,
            description,
        })
    }
}

pub fn print(filename: String) -> Result<(), Box<dyn Error>> {
    print!("{}", fs::read_to_string(filename)?);
    Ok(())
}

pub fn gen(filename: String) -> Result<(), Box<dyn Error>> {
    let current_time = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S %z"));
//    let tt = DateTime::parse_from_str("2014-11-28 21:00:09 +0800", "%Y-%m-%d %H:%M:%S %z").expect("What the hell");
    let template: String = format!("@@intro\n@@intro\n@@date\n{}\n@@date\n@@tag\n@@tag\n", current_time);
    fs::write(filename, template)?;
    Ok(())
}

pub fn gist(filename: String) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let mut flag = false;
    for line in content.lines() {
        if line.contains("@@") { flag = !flag; }
        if flag { println!("{}", line) }
    }
    Ok(())
}

pub fn parser(conf: Config) -> Result<(), Box<dyn Error>> {
    match conf.command {
        _ if conf.command == "print" => return print(conf.filename.expect("Filename is none")),
        _ if conf.command == "gist" => return gist(conf.filename.expect("Filename is none")),
        _ if conf.command == "gen" => return gen(conf.filename.expect("Filename is none")),
        _ => (),
    }
    Ok(())
}
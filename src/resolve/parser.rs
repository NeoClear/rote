use std::fs;
use std::error::Error;
use std::process;
use std::collections::VecDeque;

use chrono::prelude::*;

use crate::resolve::command;
use crate::resolve::document;

pub enum CommandType {
    File,
    SubSommand,
    Description,
}

pub fn check_cmdtype(s: String) -> CommandType {
    match s {
        _ if s.starts_with("-") => CommandType::SubSommand,
        _ if s.starts_with("\"") && s.ends_with("\"") => CommandType::Description,
        _ => CommandType::File,
    }
}

pub struct Config {
    pub command: String,
    pub sub_command: Vec<String>,
    pub filename: Vec<String>,
    pub description: Vec<String>,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        let command_list: String = String::from("print gist gen tag search help");

        let mut command: String;
        let mut sub_command: Vec<String> = Vec::new();
        let mut filename: Vec<String> = Vec::new();
        let mut description: Vec<String> = Vec::new();
        let mut tmp: String;

        let mut flag = false;
        args.next();

        let command = match args.next() {
            Some(args) => args,
            None => {
                println!("{}", document::brief_intro());
                process::exit(0);
            },
        };

        if !command_list.contains(&command) {
            return Err("Command not found");
        }

        loop {
            tmp = match args.next() {
                Some(args) => {
                    flag = true;
                    args
                },
                None => {
                    if !flag && command != "help" { return Err("Not enough arguments"); }
                    break;
                },
            };
            tmp.retain(|c| {c != '\"'});
            match check_cmdtype(tmp.clone()) {
                CommandType::File => filename.push(tmp),
                CommandType::Description => description.push(tmp),
                CommandType::SubSommand => sub_command.push(tmp),
            }
        }

        Ok(Config {
            command,
            sub_command,
            filename,
            description,
        })
    }
}



pub fn parser(conf: Config) -> Result<(), Box<dyn Error>> {
    match conf.command {
        _ if conf.command == "print" => return command::print(conf.filename, conf.sub_command, conf.description),
        _ if conf.command == "gist" => return command::gist(conf.filename, conf.sub_command, conf.description),
        _ if conf.command == "gen" => return command::gen(conf.filename, conf.sub_command, conf.description),
        _ if conf.command == "tag" => return command::tag(conf.filename, conf.sub_command, conf.description),
        _ if conf.command == "help" => return command::help(),
        _ => (),
    }
    Ok(())
}
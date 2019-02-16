use std::error::Error;
use chrono::prelude::*;
use std::fs;
use std::process;

use crate::resolve::document;
use crate::resolve::file;

pub fn print(filename: Vec<String>, sub_command: Vec<String>, description: Option<String>) -> Result<(), Box<dyn Error>> {
    for file in filename {
        print!("{}", fs::read_to_string(file)?);
    }

    Ok(())
}

pub fn gen(filename: Vec<String>, sub_command: Vec<String>, description: Option<String>) -> Result<(), Box<dyn Error>> {
    let current_time = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S %z"));
//    let tt = DateTime::parse_from_str("2014-11-28 21:00:09 +0800", "%Y-%m-%d %H:%M:%S %z").expect("What the hell");
    let template: String = format!("@@intro\n@@intro\n@@date\n{}\n@@date\n@@tag\n@@tag\n", current_time);
    for file in filename {
        fs::write(file, template.clone())?;
    }
    Ok(())
}

pub fn gist(filename: Vec<String>, sub_command: Vec<String>, description: Option<String>) -> Result<(), Box<dyn Error>> {
    for file in filename {
        let content = fs::read_to_string(file)?;
        let mut flag = false;
        for line in content.lines() {
            if line.contains("@@") { flag = !flag; }
            if flag { println!("{}", line) }
        }
    }
    Ok(())
}

pub fn tag(filename: Vec<String>, sub_command: Vec<String>, description: Option<String>) -> Result<(), Box<dyn Error>> {
    for sub in sub_command {
        match sub {
            _ if sub == "-list" => {
                let ans: Vec<file::FileList> = file::find_file("./".to_string())
                                                    .into_iter()
                                                    .filter(|s| s.filename.ends_with(".mk"))
                                                    .collect();
                for i in file::find_tag(ans) { println!("{}", i); }
            }
            _ if sub == "-gen" => {
                let ans: Vec<file::FileList> = file::find_file("./".to_string())
                                                    .into_iter()
                                                    .filter(|s| s.filename.ends_with(".mk"))
                                                    .collect();
                file::gen_rag(ans).unwrap_or_else(|err| {
                    eprintln!("Error occurred: {}", err);
                    process::exit(1);
                });
            },
            _ => (),
        }
    }
    Ok(())
}

pub fn help() -> Result<(), Box<dyn Error>>{
    println!("{}", document::help_info());
    Ok(())
}
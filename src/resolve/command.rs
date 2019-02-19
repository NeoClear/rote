use std::error::Error;
use chrono::prelude::*;
use std::fs;
use std::process;

use crate::resolve::document;
use crate::resolve::file;

pub fn print(filename: Vec<String>, sub_command: Vec<String>, description: Vec<String>) -> Result<(), Box<dyn Error>> {
    for file in filename {
        print!("{}", fs::read_to_string(file)?);
    }

    Ok(())
}

pub fn gen(filename: Vec<String>, sub_command: Vec<String>, description: Vec<String>) -> Result<(), Box<dyn Error>> {
    let current_time = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S %z"));
//    let tt = DateTime::parse_from_str("2014-11-28 21:00:09 +0800", "%Y-%m-%d %H:%M:%S %z").expect("What the hell");
    let template: String = format!("@@intro\n@@intro\n@@date\n{}\n@@date\n@@tag\n@@tag\n", current_time);
    for file in filename {
        fs::write(file, template.clone())?;
    }
    Ok(())
}

pub fn gist(filename: Vec<String>, sub_command: Vec<String>, description: Vec<String>) -> Result<(), Box<dyn Error>> {
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

pub fn tag(filename: Vec<String>, sub_command: Vec<String>, description: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut hasFile = false;
    let mut hasSub = false;
    let mut hasDes = false;

    for sub in sub_command {
        hasSub = true;
        match sub {
            _ if sub == "-list" => {
                let ans: Vec<String> = file::find_file("./".to_string())
                    .into_iter()
                    .filter(|s| s.ends_with(".md"))
                    .collect();
                for i in file::find_tag(ans) { println!("{}", i); }
                return Ok(());
            },
            _ if sub == "-gen" => {
                let ans: Vec<String> = file::find_file("./".to_string())
                    .into_iter()
                    .filter(|s| s.ends_with(".md"))
                    .collect();
                file::gen_rag(ans).unwrap_or_else(|err| {
                    eprintln!("Error occurred: {}", err);
                    process::exit(1);
                });
                return Ok(());
            },
            _ if sub == "-cache" => {
                let rag = fs::read_to_string("./rag")?;
                let mut flag = false;
                let mut job = false;
                for line in rag.lines() {
                    if line.starts_with("@@") {
//                        flag = !flag;
                        for des in description {
                            if line.clone().ends_with(des) {
                                job = true;
                                break;
                            }
                        }
                    }
                    if job {
                        if !line.starts_with("@@") {
                            println!("{}", line);
                            continue;
                        } else {
                            job = false;
                        }
                    }
                }
                return Ok(());
            },
            _ => println!("Command not recognizable."),
        }
    }

    if !hasSub {
        hasFile = true;
        let ans: Vec<String> = filename
            .into_iter()
            .filter(|s| s.ends_with(".md"))
            .collect();
        for i in file::find_tag(ans) { println!("{}", i); }
    }

    if !hasFile {

    }


    Ok(())
}

pub fn help() -> Result<(), Box<dyn Error>>{
    println!("{}", document::help_info());
    Ok(())
}
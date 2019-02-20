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
    let mut has_file = false;
    let mut has_sub = false;
    let mut has_des = false;

    for sub in sub_command {
        has_sub = true;
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
                for line in rag.lines() {
                    if line.starts_with("@@") && description.contains(&line[2..].to_string()) {
                        flag = !flag;
                        if flag { println!("{}: ", &line[2..].to_string()); }
                        continue;
                    }
                    if flag {
                        println!("{}", line);
                    }

                }
                return Ok(());
            },
            _ => println!("Command not recognizable."),
        }
    }

    if !has_sub && !filename.is_empty() {
        has_file = true;
        let ans: Vec<String> = filename
            .into_iter()
            .filter(|s| s.ends_with(".md"))
            .collect();
        for i in file::find_tag(ans) { println!("{}", i); }
    }

    if !has_file {
        let ans: Vec<String> = file::find_file("./".to_string())
                    .into_iter()
                    .filter(|s| s.ends_with(".md"))
                    .collect();
                file::gen_rag(ans).unwrap_or_else(|err| {
                    eprintln!("Error occurred: {}", err);
                    process::exit(1);
                });
        let rag = fs::read_to_string("./rag")?;
                let mut flag = false;
                for line in rag.lines() {
                    if line.starts_with("@@") && description.contains(&line[2..].to_string()) {
                        flag = !flag;
                        if flag { println!("{}: ", &line[2..].to_string()); }
                        continue;
                    }
                    if flag {
                        println!("{}", line);
                    }
                }
        return Ok(());
    }
    Ok(())
}

pub fn search(filename: Vec<String>, sub_command: Vec<String>, description: Vec<String>) -> Result<(), Box<dyn Error>> {
    let ans: Vec<String> = file::find_file("./".to_string())
        .into_iter()
        .filter(|s| {s.ends_with(".md")})
        .collect();
    for des in description {
        for f in ans.clone() {
            let p = file::search(f.clone(), des.clone());
            if !p.is_empty() { println!("{}: \n{:#?}", f.clone(), p); }
        }
    }
    Ok(())
}

pub fn help() -> Result<(), Box<dyn Error>>{
    println!("{}", document::help_info());
    Ok(())
}
use std::fs;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FileList {
    pub filename: String,
    pub is_file: bool,
}

//pub struct Tag {
//    pub tagname: String,
//    pub filename: Vec<String>,
//}

impl FileList {
    pub fn to_string(&self) -> String {
        self.filename.clone()
    }
}

pub fn find_file(p: String) -> Vec<String>{
    let mut flist: Vec<FileList> = Vec::new();
    let mut ans: Vec<String> = Vec::new();
    let mut res: Vec<String> = Vec::new();
    for path in fs::read_dir(p.clone()).unwrap() {
        flist.push(FileList {
            filename: path.unwrap().path().to_str().unwrap().to_string(),
            is_file: false,
        });
    }
    for (index, path) in fs::read_dir(p.clone()).unwrap().enumerate() {
        flist[index].is_file = path.unwrap().file_type().unwrap().is_file();
    }
    for l in flist.iter() {
        if !l.is_file {
            ans.append(&mut find_file(l.filename.to_string()));
        }
    }
    res.append(&mut ans);
    for f in flist {
        res.push(f.filename);
    }
    res
}

pub fn get_tag(f: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut flag = false;
    let mut ans: Vec<String> = Vec::new();
    let content = fs::read_to_string(f)?;
    for line in content.lines() {
        if line.to_string() == "@@tag" { flag = !flag; continue; }
        if flag && !line.to_string().is_empty() {
            ans.push(line.to_string());
        }
    }
    Ok(ans)
}

pub fn find_tag(v: Vec<String>) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    for f in v {
        for s in get_tag(f).unwrap() {
            if !tags.contains(&s) { tags.push(s); }
        }
    }
    tags
}

pub fn gen_rag(v: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut rag: HashMap<String, Vec<String>> = HashMap::new();
    for f in v {
        for i in get_tag(f.clone()).unwrap() {
            let t = rag.entry(i).or_insert(Vec::new());
            t.push(f.clone());
        }
    }
    let mut content: String = String::new();
    for (tag, vc) in rag {
        content = format!("{}@@{}\n", content, tag);
        for f in vc {
            content = format!("{}{}\n", content, f);
        }
        content = format!("{}@@{}\n", content, tag);
    }
    fs::write("rag", content)?;
    Ok(())
}

pub fn get_filename(tag: String) -> Vec<String> {
    Vec::new()
}
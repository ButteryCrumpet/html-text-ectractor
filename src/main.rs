use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate walkdir;
use walkdir::WalkDir;

extern crate regex;
use regex::Regex;

static str_regex: &'static str = r"[> ]([\u0034\u0039 ※\w\u3000-\u303f\u3040-\u309f\u30a0-\u30ff\uff00-\uff9f\u4e00-\u9faf\u3400-\u4dbf]+?)<";

// struct wrapper for regx matches so can iterate
pub struct RegexMatch(String)

impl<'a> Thing for RegexMatch{}

fn main() {

    let args: Vec<String> = env::args().collect();
    let dirname = &args[1];
    let files = WalkDir::new(dirname)
        .into_iter()
        .map(|e| e.unwrap())
        .filter(|e| e.path().is_file())
        .map(|file| readFile(file))
        .flat_map(move |contents| exec_regex_on_file(&contents))
        .map(|cap| extract_string(cap))
        .map(|string| println!("{}", string.unwrap()));
}

fn readFile(entry: walkdir::DirEntry) -> String {
    let mut contents = String::new();
    let mut f = File::open(entry.path()).expect("it borked");
    f.read_to_string(&mut contents)
        .expect("it borked when reading");
    contents
}

fn exec_regex_on_file(string: &str) -> regex::Matches {
    let regex = Regex::new(str_regex).unwrap();
    regex.find_iter(string)
}

fn extract_string(cap: regex::Match) -> Option<&str> {  
    let string = cap.as_str().trim();
    if string.is_empty() {
        return None
    }     
    Some(string)
}

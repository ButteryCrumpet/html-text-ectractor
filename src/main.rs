use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeSet;

extern crate walkdir;
use walkdir::WalkDir;

extern crate regex;
use regex::Regex;

// struct wrapper for regx matches so can iterate
struct RegexMatches {
    matches: Vec<String>,
}

impl RegexMatches {
    fn new (string: String, regex: &regex::Regex) -> RegexMatches {
        let to_trim: &[_] = &[' ', '<', '>'];
        let matches: Vec<String> = regex.find_iter(&string)
            .map(|m| m.as_str().trim_matches(to_trim).trim().to_string())
            .collect();

        RegexMatches {
            matches: matches
        }
    }

}

impl IntoIterator for RegexMatches {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.matches.into_iter()
    }
}

fn main() {
    let str_regex = r"[> ]([\u0034\u0039 ※\w\u3000-\u303f\u3040-\u309f\u30a0-\u30ff\uff00-\uff9f\u4e00-\u9faf\u3400-\u4dbf]+?)<";
    let regex = Regex::new(str_regex).unwrap();
    let args: Vec<String> = env::args().collect();
    let dirname = &args[1];

    let text_set = WalkDir::new(dirname)
        .into_iter()
        .map(|e| e.unwrap())
        .filter(|e| e.path().is_file())
        .map(|file| read_file(file))
        .flat_map(|contents| RegexMatches::new(contents, &regex))
        .filter(|s| !s.is_empty())
        .collect::<BTreeSet<String>>();

    let mut deduped: Vec<String> = Vec::new();
    for text in text_set {
        match deduped.last() {
            Some(x) => {
                if text == x.to_owned() {
                    continue;
                }
            }
            _ => ()
        }
        deduped.push(text);
    }

    println!("Japanese");
    for text in deduped {
        println!("{}", text);
    }
}

fn read_file(entry: walkdir::DirEntry) -> String {
    let mut contents = String::new();
    let mut f = File::open(entry.path()).expect("it borked");
    f.read_to_string(&mut contents)
        .expect("it borked when reading");
    contents
}

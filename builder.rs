use std::io::prelude::*;
use std::fs;


const URL: &str = "\
    https://raw.githubusercontent.com\
    /dwyl/english-words\
    /master/words_alpha.txt\
";


const ELM_START: &[u8] =
r#"module Main exposing
    (list)

data = ""#.as_bytes();

const ELM_END: &[u8] =
r#""
list : List
list =
    String.split ";" data
"#.as_bytes();


const SEMICOLON: &[u8] =
";".as_bytes();


fn main() {
    fs::DirBuilder::new()
        .create("src").unwrap_or_default();
    let mut file = fs::File::create("src/EnglishWords.elm")
        .expect("Can't open EnglishWords.elm");

    file.write(ELM_START).unwrap();

    let words = load_words();
    for word in words.split("\r\n") {
        file.write(word.as_bytes()).unwrap();
        file.write(SEMICOLON).unwrap();
    }

    file.write(ELM_END).unwrap();
}


fn load_words() -> String {
    reqwest::blocking::get(URL)
        .expect("Could not download word list")
        .text()
        .expect("Could not decode text")
}

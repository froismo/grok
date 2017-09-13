extern crate glob;

use glob::glob;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::fmt;

fn main() {

    let mut output = String::new();

    fmt::write(&mut output, format_args!("pub static PATTERNS: &[(&'static str, &'static str)] = &[\n")).unwrap();


    for line in glob("patterns/*.pattern").unwrap() // load filepaths
        // extract the filepath
        .map(|e| e.unwrap())
        // open file for path
        .map(|path| File::open(path).unwrap()) 
        // flatten to actual lines
        .flat_map(|f| BufReader::new(f).lines()) 
        .map(|line| line.unwrap())
        // filter comments
        .filter(|line| !line.starts_with("#"))
        // filter empty lines
        .filter(|line| !line.is_empty())
    {
        let (key, value) = line.split_at(line.find(" ").unwrap());
        fmt::write(&mut output, format_args!("\t(\"{}\", r###\"{}\"###),\n", key, &value[1..])).unwrap();
    }

    fmt::write(&mut output, format_args!("];\n")).unwrap();

    let mut file = File::create("src/patterns.rs").unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

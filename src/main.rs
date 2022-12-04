pub mod day3;

use std::{io::{Lines, BufReader, BufRead}, fs::File};

fn file_lines_iter(filename : &str) -> Lines<BufReader<File>>{
    
    let file;
    match File::open(&filename) {
        Ok(value) => file = value,
        Err(e) => panic!("{e}"),
    }
    BufReader::new(file).lines()
}

fn main() {
    crate::day3::day3("input_day3.txt");
}

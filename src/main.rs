#![feature(iter_array_chunks)]

pub mod day6;

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
    crate::day6::day6("input_day6.txt");
}

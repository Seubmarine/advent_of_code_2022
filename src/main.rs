#![feature(iter_array_chunks)]

pub mod day4;

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
    crate::day4::day4("input_day4.txt");
}

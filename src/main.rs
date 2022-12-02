mod day2;

use std::{fs::File, io::{BufRead, BufReader}, process::exit};

#[derive(Default)]
struct Inventory {
    calories : u64
}

fn day1() {
    let filepath = match std::env::args().nth(1) {
        Some(value) => value,
        None => "input_day1.txt".to_string(),
    };
    let file;
    match File::open(&filepath) {
        Ok(value) => file = value,
        Err(e) => {eprintln!("\"{filepath}\": {e}"); exit(2)},
    }
    let lines = BufReader::new(file).lines();
    
    let mut current = Inventory::default();
    let mut inventories : Vec<Inventory> = Vec::new();
    
    for line_r in lines {
        if let Ok(line) = line_r {
            if line.is_empty() {
                inventories.push(current);
                current = Inventory::default();
            }
            else {
                current.calories += line.parse::<u64>().unwrap();                
            }
        } else {
            panic!("Error reading line")
        }
    }
    inventories.sort_by_key(|k| std::cmp::Reverse(k.calories));    
    
    if !inventories.is_empty() {
        println!("Solution 1 {}", inventories[0].calories);
    }
    if inventories.len() >= 3 {
        println!("Solution 2 {}", inventories[0].calories + inventories[1].calories + inventories[2].calories);
    }
}

fn main() {
    // day1();
    crate::day2::day2();
}

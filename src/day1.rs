use std::{fs::File, io::{BufRead, BufReader, Lines}, process::exit, env::args};

#[derive(Default)]
struct Inventory {
    calories : u64
}

fn day1(filename : &str) {
    let lines = file_lines_iter(filename);
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
use std::{io::{BufRead, BufReader}, str::FromStr};

#[derive(PartialEq, Eq)]
enum Shape {
	Rock = 1,
	Paper = 2,
	Scisor = 3
}

trait Score {
	fn score(&self) -> u32;
}

impl Score for Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scisor => 3,
        }
    }
}

trait ShapeLoose {
	
}

impl FromStr for Shape {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"X" => Ok(Self::Rock),
			"Y" => Ok(Self::Paper),
			"Z" => Ok(Self::Scisor),
			"A" => Ok(Self::Rock),
			"B" => Ok(Self::Paper),
			"C" => Ok(Self::Scisor),
			_ => Err("Not a valid Shape")
		}
    }
}

use std::cmp::Ordering;
impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Shape::Rock, Shape::Paper) => Some(Ordering::Less),
            (Shape::Rock, Shape::Scisor) => Some(Ordering::Greater),
            (Shape::Paper, Shape::Rock) => Some(Ordering::Greater),
            (Shape::Paper, Shape::Scisor) => Some(Ordering::Less),
            (Shape::Scisor, Shape::Rock) => Some(Ordering::Less),
            (Shape::Scisor, Shape::Paper) => Some(Ordering::Greater),
			(_, _) => Some(Ordering::Equal)
		}
    }
}

pub fn day2() {
	let filepath = match std::env::args().nth(1) {
        Some(value) => value,
        None => "input_day2.txt".to_string(),
    };
    let file;
    match std::fs::File::open(&filepath) {
        Ok(value) => file = value,
        Err(e) => {eprintln!("\"{filepath}\": {e}"); std::process::exit(2)},
    }
    let lines = BufReader::new(file).lines();
    
	let mut score = 0;
	for line in lines {
		match line {
			Err(e) => {eprintln!("{e}"); return;},
			Ok(line) => {
				let (opponent, player) = line.split_once(' ').expect("Line couldn't be split in two");
				let opponent = opponent.parse::<Shape>().expect("Invalid Shape for the opponent");
				let player = player.parse::<Shape>().expect("Invalid Shape for the player");
				
				let battle_score =  match player.partial_cmp(&opponent).unwrap() {
					Ordering::Less => 0,
					Ordering::Equal => 3,
					Ordering::Greater => 6,
				};
				score += battle_score + player.score();
			}
		} 
	}
	println!("Solution 1: {score}");
}
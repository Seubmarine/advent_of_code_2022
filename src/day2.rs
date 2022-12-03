use std::{io::{BufRead, BufReader}, str::FromStr};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
	Rock = 1,
	Paper = 2,
	Scisor = 3
}

impl Shape {
    fn shape_score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scisor => 3,
        }
    }

	fn lose_against(&self) -> Shape {
		match self {
			Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scisor,
            Shape::Scisor => Shape::Rock
		}
	}

	fn win_agaist(&self) -> Shape {
		match self {
			Shape::Rock => Shape::Scisor,
            Shape::Paper => Shape::Rock,
            Shape::Scisor => Shape::Paper
		}
	}

	fn score_against(&self, other : &Self) -> u32
	{
		match self.partial_cmp(&other).unwrap() {
			Ordering::Less => 0,
			Ordering::Equal => 3,
			Ordering::Greater => 6,
		}
	}
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

enum Goal {
	Loss,
	Draw,
	Win
}

impl FromStr for Goal {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"X" => Ok(Self::Loss),
			"Y" => Ok(Self::Draw),
			"Z" => Ok(Self::Win),
			_ => Err("Invalid goal argument")
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
    
	let mut score_1 = 0;
	let mut score_2 = 0;
	for line in lines {
		match line {
			Err(e) => {eprintln!("{e}"); return;},
			Ok(line) => {
				let (opponent_str, player_str) = line.split_once(' ').expect("Line couldn't be split in two");
				let opponent = opponent_str.parse::<Shape>().expect("Invalid Shape for the opponent");
				let mut player = player_str.parse::<Shape>().expect("Invalid Shape for the player");
				
				score_1 += player.score_against(&opponent) + player.shape_score();
				let goal = player_str.parse::<Goal>().unwrap();
				match goal {
					Goal::Draw => player = opponent,
					Goal::Win => player = opponent.lose_against(),
					Goal::Loss => player = opponent.win_agaist(),
				}
				score_2 += player.score_against(&opponent) + player.shape_score();
			}
		}
	}
	println!("Solution 1: {score_1}");
	println!("Solution 2: {score_2}");
}
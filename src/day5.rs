use std::{str::FromStr, num};

use itertools::{multiunzip, Itertools, MultiUnzip};

#[derive(Debug)]
struct Instruction {
	move_count : usize,
	from : usize,
	to : usize
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("move") {
			return Err(());
		}
		let mut number_iter = s.split_whitespace().into_iter()
		.filter_map(|word| word.parse().ok());
		Ok(Instruction { 
			move_count: number_iter.next().unwrap(), 
			from: number_iter.next().unwrap(), 
			to: number_iter.next().unwrap()
		})

    }
}

fn into_crate(line : &str) -> Vec<Option<char>> {
	let mut individual = line.chars();
	debug_assert!(individual.next().is_some());
	let boxes : Vec<Option<char>> = individual.step_by(4)
	.map(|c| 
		match c {
			' ' => None,
			'A'..='Z' => Some(c),
			_ => panic!("Invalid char into table")
		}).collect();
	boxes
}

fn multivec_unzip(v : Vec<Vec<Option<char>>>) -> Vec<Vec<char>> {
	let mut new_v : Vec<Vec<char>> = Vec::new();
	new_v.reserve(v.len());

	for _ in 0..v[0].len() {
		new_v.push(Vec::new());
	}

	for horizon in v.into_iter().rev() {
		for (j, vectical) in horizon.into_iter().enumerate() {
			if vectical.is_some() {
				new_v[j].push(vectical.unwrap());
			}
		}
	}
	new_v
}

fn debug_vec_vec(v : &Vec<Vec<char>>) {
	for i in v {
		for j in i {
			print!("{j:?} ");
		}
		println!();
	}
}

pub fn day5(filename : &str) {
	let file = std::fs::read_to_string(filename).unwrap();
	let lines = file.as_str().lines();
	
	let table : Vec<Vec<Option<char>>> = lines.clone()
	.filter(|l| !l.starts_with("move") && !l.starts_with(" 1") && !l.is_empty())
	.inspect(|l| println!("{l}"))
	.map(into_crate).collect();

	let mut test = multivec_unzip(table);
	// debug_vec_vec(&test);

	let instructions = lines
	.filter_map(|s| s.parse::<Instruction>().ok());

	debug_vec_vec(&test);
	for instruction in instructions {
		// for i in 0..instruction.move_count {
		// 	let tmp = &test[instruction.from - 1].pop().unwrap();
		// 	test[instruction.to - 1].push(*tmp);
		// }
		let v_size = test[instruction.from - 1].len();
		let mut tmp = test[instruction.from - 1].drain(v_size - instruction.move_count..).collect_vec();
		test[instruction.to - 1].append(&mut tmp);
		println!();
		debug_vec_vec(&test);
	}
}
use std::{str::FromStr, num::ParseIntError};

struct Assignement {
	begin : u32,
	end : u32,
}

impl Assignement {
	fn contain(&self, other: &Self) -> bool{
		self.begin <= other.begin && self.end >= other.end
	}
}

impl FromStr for Assignement {
	type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left_number, right_number) = s.split_once('-').expect("Missing - separator");
		Ok(Self { begin : left_number.parse::<u32>()?, end : right_number.parse::<u32>()?})
	}
}

pub fn day4(filename : &str) {
	let file = std::fs::read_to_string(filename).unwrap();
	let lines = file.as_str().lines();

	let solution_one : u32 = lines.map(|line| line.split_once(',').unwrap())
	.map(|(l, r)| (l.parse::<Assignement>().unwrap(), r.parse::<Assignement>().unwrap()))
	.map(|(l, r)| (l.contain(&r) || r.contain(&l)) as u32)
	.sum();

	println!("Solution 1: {solution_one}");
}
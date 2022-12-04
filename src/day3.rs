use std::fs::read_to_string;

#[derive(Debug)]
enum AocError {
	NoDouble,
	InvalidChar
}

fn alphabet_to_priorities(n : u8) -> Result<u8, AocError>{
	match n {
		b'a'..=b'z' => Ok(n - b'a' + 1),
		b'A'..=b'Z' => Ok(n - b'A' + 27),
		_ => Err(AocError::InvalidChar),
	}
}

fn line_to_number(line : &str) -> Result<u8, AocError> {
	let (r, l) = line.split_at(line.len() / 2);
	for byte in r.bytes() {
		if l.contains(byte as char) {
			return alphabet_to_priorities(byte);
		}
	}
	return Err(AocError::NoDouble);
}

fn map_all_priorities_to_table(line : &str, table : &mut[u32; 52]) {
	let mut temporary_table = [0; 52];
	for byte in line.bytes().map(|b| alphabet_to_priorities(b).unwrap() - 1) {
		if temporary_table[byte as usize] == 0 {
			temporary_table[byte as usize] += 1;
		}
	}
	for (index, data) in table.iter_mut().enumerate() {
		*data += temporary_table[index];
	}
}

pub fn day3(filename : &str) {
	let file = read_to_string(filename).unwrap();
	let lines = file.as_str().lines();
	let soluce_one : i32 = lines.clone().map(line_to_number)
		.map(|n| n.unwrap() as i32)
		.sum();
	println!("Solution 1 : {soluce_one}");

	let mut soluce_two = 0;
	let line_by_three = lines.array_chunks();
	for [one, two, three] in line_by_three {
		let mut table = [0 ; 52];
		map_all_priorities_to_table(&one, &mut table);
		map_all_priorities_to_table(&two, &mut table);
		map_all_priorities_to_table(&three, &mut table);
		for (index, value) in table.iter().enumerate() {
			if value == &3 {soluce_two += index + 1; break;}
		}
	}
	println!("Solution 2 : {soluce_two}");
}
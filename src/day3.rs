#[derive(Debug)]
enum AocError {
	NoDouble,
	InvalidChar
}

fn line_to_number(line : String) -> Result<u8, AocError> {
	let (r, l) = line.split_at(line.len() / 2);
	for chara in r.chars() {
		if l.contains(chara) {
			println!("{chara}");
			return match chara.into() {
				'a'..='z' => Ok(chara as u8 - b'a' + 1),
				'A'..='Z' => Ok(chara as u8 - b'A' + 27),
				_ => Err(AocError::InvalidChar),
			}
		}
	}
	return Err(AocError::NoDouble);
}

pub fn day3(filename : &str) {
	let lines = crate::file_lines_iter(filename);

	let all : i32 = lines.map(|l| l.unwrap())
		.map(line_to_number)
		.map(|n| n.unwrap() as i32)
		.inspect(|n| println!("{n}"))
		.sum();
	println!("Solution 1 : {all}");
}
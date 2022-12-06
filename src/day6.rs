use itertools::Itertools;

struct PeekableChunk<'a> { 
	data : &'a str,
	chunk_size : usize,
	current : usize
}

#[derive(Debug)]
enum PeekableChunkError {
	NoUnique
}

impl<'a> PeekableChunk<'a> {
    fn new(data: &'a str, chunk_size: usize) -> Self {
		Self { chunk_size, data, current : 0 }
	}

	fn first_unique_index(self) -> Result<usize, PeekableChunkError> {
		let chunk_size = self.chunk_size;
		
		for (i, chunk) in self.enumerate() {
			if chunk.chars().unique().count() == chunk_size {
				return Ok(i + chunk_size);
			};
		}
		Err(PeekableChunkError::NoUnique)
	}
}

impl<'a> Iterator for PeekableChunk<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current + self.chunk_size >= self.data.len() {return None};
		let res = &self.data[self.current..self.current + self.chunk_size];
		self.current += 1;
		Some(res)
    }
}

pub fn day6(filename : &str) {
	let file = std::fs::read_to_string(filename).unwrap();

	let peek_chunk_packet = PeekableChunk::new(&file, 4).first_unique_index().unwrap();
	println!("Solution 1 : {peek_chunk_packet}");
	let peek_chunk_message = PeekableChunk::new(&file, 14).first_unique_index().unwrap();
	println!("Solution 2 : {peek_chunk_message}");
}
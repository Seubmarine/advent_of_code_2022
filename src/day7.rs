use std::fmt::Display;

use itertools::Itertools;

trait DisplayDepth
	where Self : std::fmt::Display
{
	fn fmt_depth(&self, f: &mut std::fmt::Formatter<'_>, depth : usize) -> std::fmt::Result {
		for _ in 0..depth {
			write!(f, "    ")?;
		};
		self.fmt(f)
	}
}

#[derive(Debug)]
struct File {
	name : String,
	size : usize
}

impl File {
    fn new(name: String, size: usize) -> Self { Self { name, size } }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- {} (file, size={})", self.name, self.size)
    }
}

impl DisplayDepth for File {}

#[derive(Debug)]
struct Directory {
	name : String,
	inner : Vec<FileSystem>
}

impl Directory {
    fn new(name: String) -> Self { Self { name, inner : Vec::new() } }

	fn push_command(&mut self, mut commands : &mut dyn Iterator<Item = Command>)
	{
		while let Some(command) = commands.next() {
			match command {
				Command::List => panic!("Shoulnt have a list"),
				Command::ChangeDirectory(path) => {
					if path.as_str().cmp("..").is_eq() {
						break ;
					}
					else {
						let dir_to_move = self.inner.iter_mut()
						.find(|fd| 
							match fd {
								FileSystem::Directory(dir) => dir.name.cmp(&path).is_eq(),
								FileSystem::File(_) => false,
							}
						);
						if dir_to_move.is_some() {
							let dir_to_move = dir_to_move.unwrap().into_mut_directory().unwrap();
							dir_to_move.push_command(commands);
						}
					}
				},
				Command::Listed(filesystem) => {
					self.inner.push(filesystem);
				}
			}
		}
	}

	fn get_size(&self, vec : &mut Vec<usize>) -> usize {
		let mut size = 0;
		for fs in self.inner.iter() {
			size += match fs {
				FileSystem::Directory(dir) => dir.get_size(vec),
				FileSystem::File(file) => file.size,
			}
		}
		vec.push(size);
		size
	}
}

impl std::fmt::Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- {} (dir)", self.name)
    }
}

impl DisplayDepth for Directory {
    fn fmt_depth(&self, f: &mut std::fmt::Formatter<'_>, depth : usize) -> std::fmt::Result {
		for _ in 0..depth {
			write!(f, "    ")?;
		}
		self.fmt(f)?;
		for filesystem in &self.inner {
			filesystem.fmt_depth(f, depth)?
		}
		Ok(())
	}
}

#[derive(Debug)]
enum FileSystem {
	Directory(Directory),
	File(File)
}

impl Into<FileSystem> for File {
    fn into(self) -> FileSystem {
        FileSystem::File(self)
    }
}

impl Into<FileSystem> for Directory {
    fn into(self) -> FileSystem {
        FileSystem::Directory(self)
    }
}

impl FileSystem {
	fn append_filesystem(&mut self, other : Self){
		match self {
			FileSystem::Directory(dir) => dir.inner.push(other),
			FileSystem::File(_) => panic!("Trying to append to a file"),
		}
	}

	fn into_directory(&self) -> Option<&Directory> {
		match self {
			FileSystem::Directory(dir) => Some(dir),
			FileSystem::File(_) => None,
		}
	}

	fn into_mut_directory(&mut self) -> Option<&mut Directory> {
		match self {
			FileSystem::Directory(dir) => Some(dir),
			FileSystem::File(_) => None,
		}
	}

	fn new() -> Self {FileSystem::Directory(Directory::new("/".to_string()))}
}

impl DisplayDepth for FileSystem {
    fn fmt_depth(&self, f: &mut std::fmt::Formatter<'_>, depth : usize) -> std::fmt::Result {
		match &self {
			FileSystem::Directory(dir) => dir.fmt_depth(f, depth + 1),
			FileSystem::File(file) => file.fmt_depth(f, depth + 1),
		}
	}
}

impl std::fmt::Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_depth(f, 0)
    }
}

#[derive(Debug)]
enum Command {
	ChangeDirectory(String),
	List,
	Listed(FileSystem)
}

impl Command {
	fn new(command_line : &str) -> Result<Command, ()> {
		if command_line.starts_with("$ cd") {
			let command_line = command_line[5..].to_string();
			Ok(Command::ChangeDirectory(command_line.to_string()))
		}
		else if command_line.starts_with("$ ls") {
			Ok(Command::List)
		}
		else {
			let (begin, end) = command_line.split_once(' ').unwrap();
			if begin.cmp("dir").is_eq() { 
				Ok(Command::Listed(Directory::new(end.to_string()).into()))
			}
			else {
				Ok(Command::Listed(File::new(end.to_string(), begin.parse::<usize>().unwrap()).into()))
			}
		}
	}
}

pub fn day7(filename : &str) {
// 	let dir1 = Directory::new("/".into());
// 	let file1 = File::new("i".into(), 42);
// 	let file2 = File::new("file2".into(), 56);
	
// 	let mut dir2 : FileSystem = Directory::new("usr".into()).into();
// 	let userfile = File::new("Seubmarine".into(), 256);
// 	dir2.append_filesystem(userfile.into());
// 	let mut system : FileSystem = dir1.into();
// 	system.append_filesystem(file1.into());
// 	system.append_filesystem(file2.into());
// 	system.append_filesystem(dir2);
// 	println!("{system}")
	let file = std::fs::read_to_string(filename).unwrap();
	let lines = file.as_str().lines();

	let commands : Vec<Command> = lines.map(|line| Command::new(line).unwrap()).filter(|cmd| match cmd { Command::List => false, _ => true}).collect();

	let mut root = Directory::new("/".to_string());
	root.push_command(&mut commands.into_iter());
	
	let mut dirs_size: Vec<usize> = Vec::new();
	let _ = root.get_size(&mut dirs_size);

	let system = FileSystem::Directory(root);
	println!("{system}");

	let i : usize = dirs_size.clone().into_iter().filter(|x| x <= &100000).sum();
	println!("Solution 1: {i}");
	dirs_size.sort();
	let space_used = dirs_size.last().unwrap();
	const SPACE_TOTAL : usize = 70000000;
	let space_needed : usize = SPACE_TOTAL - space_used;
	println!("space_needed {space_needed}");
	let need_to_free = 30000000 - space_needed;
	println!("need_to_free {need_to_free}");
	let dir_to_remove = dirs_size.into_iter().filter(|x| x > &need_to_free).next().unwrap();
	// for i in dir_to_remove {
	// 	println!("{i}");
	// }
	println!("Solution 2: {dir_to_remove}");
}
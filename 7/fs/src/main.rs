//use id_tree::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Debug)]
enum INode {
	File {
		//		name: String,
		size: u32,
	},
	Dir {
		//		name: String,
		children: HashSet<PathBuf>,
	},
}
use INode::*;

impl INode {
	fn is_file(&self) -> bool {
		match self {
			File { .. } => true,
			_ => false,
		}
	}

	fn is_dir(&self) -> bool {
		match self {
			Dir { .. } => true,
			_ => false,
		}
	}
}

#[derive(Debug)]
struct Env {
	cwd: PathBuf,
	fs: HashMap<PathBuf, INode>,
}

impl Env {
	// adds ONE level of directory to our knowledge base
	fn add_directory(&mut self, path: &str) {
		let mut dir = self.cwd.to_owned();
		dir.push(path);
		{
			match dir.parent() {
				None => (),
				Some(parent) => {
					let parent = self.fs.get_mut(parent).unwrap();
					match parent {
						Dir { children, .. } => children.insert(PathBuf::from(path)),
						_ => panic!("subdirectory of non-directory"),
					};
				}
			}
		}
		self.fs.entry(dir).or_insert(Dir {
			//			name: path.to_string(),
			children: HashSet::new(),
		});
	}

	fn add_file(&mut self, path: &str, size: u32) {
		let mut filename = self.cwd.to_owned();

		filename.push(path);
		{
			let parent = self.fs.get_mut(filename.parent().unwrap()).unwrap();
			match parent {
				Dir { children, .. } => children.insert(PathBuf::from(path)),
				_ => panic!("subdirectory of non-directory"),
			};
		}
		self.fs.entry(filename).or_insert(File {
			//			name: path.to_string(),
			size: size,
		});
	}

	fn change_dir(&mut self, new_path: &str) {
		if new_path == ".." {
			self.cwd.pop();
		} else {
			self.add_directory(new_path);
			self.cwd.push(new_path);
		}
	}

	fn size(&self, path: &Path) -> u32 {
		match &self.fs[path] {
			File { size, .. } => *size,
			Dir { children, .. } => children.iter().map(|c| self.size(&path.join(c))).sum(),
		}
	}
}

fn process_chunk<'a, I>(mut chunk: I, env: &mut Env)
where
	I: Iterator<Item = &'a str>,
{
	let s = chunk.next().unwrap();
	if s.starts_with("$ ls") {
		for line in chunk {
			let space = line.find(' ').unwrap();
			let path = &line[(space + (1 as usize))..];
			let size_or_dir = &line[..space];
			if size_or_dir == "dir" {
				env.add_directory(path);
			} else {
				env.add_file(path, size_or_dir.parse::<u32>().unwrap());
			}
		}
	} else if s.starts_with("$ cd") {
		env.change_dir(&s[5..]);
	} else {
		panic!("bad command {s}");
	}
}

fn main() {
	// every ^$ line starts a new chunk
	let mut i = 0;
	let input = include_str!("../input.txt").lines().group_by(|l| {
		if l.starts_with('$') {
			i += 1;
		}
		return i;
	});
	let mut env = Env {
		cwd: PathBuf::from("/"),
		fs: HashMap::new(),
	};
	env.add_directory("/");
	for (_, chunk) in input.into_iter() {
		process_chunk(chunk, &mut env);
	}
	let total_size = env.size(Path::new("/"));
	let mut answer1 = 0;
	for path in env.fs.keys() {
		if env.fs[path].is_dir() {
			let size = env.size(path);
			dbg!(path, size);
			if size <= 100000 {
				answer1 += size
			}
		}
	}
	println!("{answer1}");
	let size_limit = 70000000 - 30000000;
	let mut candidate = PathBuf::new();
	let mut candidate_size = total_size;
	for path in env.fs.keys() {
		if env.fs[path].is_dir() {
			let size = env.size(path);
			if size > total_size - size_limit && size < candidate_size {
				candidate = path.to_path_buf();
				candidate_size = size;
			}
		}
	}
	println!("{} {candidate_size}", candidate.display());
}

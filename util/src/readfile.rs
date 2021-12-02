use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

pub fn read_ok_lines<P>(filename: P) -> Box<dyn Iterator<Item = String>>
where P: AsRef<Path>, {
	let lines = read_lines(filename).ok().unwrap();
	Box::new(lines.filter_map(|line| line.ok()))
}
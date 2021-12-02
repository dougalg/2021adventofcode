use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
	let mut result = 0;
	let mut prev = None;
	if let Ok(lines) = read_lines("./input.txt") {
		for line in lines {
			if let Ok(ip) = line {
				let val = ip.parse::<i128>().unwrap();
				if let Some(actual_prev) = prev {
					if actual_prev < val {
						result = result + 1;
					}
				}
				prev = Some(val);
			}
		}
	}
	println!("{}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
	let mut horizontal_distance = 0;
	let mut vertical_distance = 0;
	if let Ok(lines) = read_lines("./input.txt") {
		for line in lines {
			if let Ok(actual_line) = line {
				let mut split_line = actual_line.split_whitespace();
				let command = split_line.next();
				let value = split_line.next().unwrap().parse::<i128>().unwrap();
				match command {
					Some("forward") => {
						horizontal_distance += value;
					}
					Some("down") => {
						vertical_distance += value;
					}
					Some("up") => {
						vertical_distance -= value;
					}
					Some(&_) => {}
					None => {}
				}
			}
		}
	}
	println!("{} {} {}", horizontal_distance, vertical_distance, horizontal_distance * vertical_distance);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

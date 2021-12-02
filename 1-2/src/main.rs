mod triple_iter;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
	let mut result = 0;
	let mut prev = None;
	if let Ok(lines) = read_lines("./input.txt") {
		let x = triple_iter::TripleIter::new(lines.filter_map(|line| line.ok()))
			.map(window_to_sum);
		for line in x {
			if let Some(actual_prev) = prev {
				if actual_prev < line {
					result = result + 1;
				}
			}
			prev = Some(line);
		}
	}
	println!("{}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

fn window_to_sum(item: [String; 3]) -> i128 {
	item.iter()
		.filter_map(|str_val| str_val.parse::<i128>().ok())
		.reduce(|a, b| a + b)
		.unwrap()
}

use util::readfile::read_ok_lines;

fn main() {
	let mut horizontal_distance = 0;
	let mut vertical_distance = 0;
	for line in read_ok_lines("./input.txt") {
		let mut split_line = line.split_whitespace();
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
	println!("{} {} {}", horizontal_distance, vertical_distance, horizontal_distance * vertical_distance);
}

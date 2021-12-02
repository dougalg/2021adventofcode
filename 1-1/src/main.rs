use util::window::Window;
use util::readfile::read_ok_lines;

fn main() {
	let result = read_ok_lines("./input.txt")
		.filter_map(|line| line.parse::<i128>().ok());
	let final_result = Window::new(result, 2)
		.map(|pair| { if pair[0] < pair[1] { 1 } else { 0 } })
		.reduce(| a, b | { a + b });

	println!("{}", final_result.unwrap());
}

use util::window::Window;
use util::readfile::read_ok_lines;

fn main() {
	let mut result = 0;
	let mut prev = None;

	let x = Window::new(read_ok_lines("./input.txt"), 3)
		.map(window_to_sum);
	for line in x {
		if let Some(actual_prev) = prev {
			if actual_prev < line {
				result = result + 1;
			}
		}
		prev = Some(line);
	}
	println!("{}", result);
}

fn window_to_sum(item: Vec<String>) -> i128 {
	item.iter()
		.filter_map(|str_val| str_val.parse::<i128>().ok())
		.reduce(|a, b| a + b)
		.unwrap()
}

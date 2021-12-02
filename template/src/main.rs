use util::readfile::read_ok_lines;

fn main() {
	let result = read_ok_lines("./input.txt");

	println!("{}", result);
}

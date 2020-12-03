use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("d03-input").unwrap();
	let mut input = String::new();
	file.read_to_string(&mut input).unwrap();

	let (mut tree, mut col) = (0, 0);
	for line in input.lines() {
		let x = line.chars().nth(col);
		if x.unwrap() == '#' {
			tree += 1;
		}
		col = (col + 3) % line.len();
	}
	println!("Number of trees encountered: {}", tree);
}
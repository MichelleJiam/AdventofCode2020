use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("d03-input").unwrap();
	let mut input = String::new();
	file.read_to_string(&mut input).unwrap();

	let mut tree_count = tree_counter(1, &input, 1);
	let mut x_inc = 3;
	while x_inc <= 7 {
		tree_count *= tree_counter(x_inc, &input, 1);
		x_inc += 2;
	}
	tree_count *= tree_counter(1, &input, 2);
	println!("Number of trees encountered: {}", tree_count);
}

fn tree_counter(x_inc: usize, input: &str, y_inc: usize) -> u128 {
	let (mut tree, mut col) = (0, 0);
	for line in input.lines().step_by(y_inc) {
		let x = line.chars().nth(col);
		if x.unwrap() == '#' {
			tree += 1;
		}
		col = (col + x_inc) % line.len();
	}
	println!("Tree count is {}", tree);
	return tree;
}
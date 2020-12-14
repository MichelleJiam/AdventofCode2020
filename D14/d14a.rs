use std::fs::read_to_string;

fn main() {
	let input = read_to_string("d14-input").expect("something went wrong reading file");
	let mut data: Vec<i32> = input.lines().map(|n| n.parse().unwrap()).collect();
	
	data.sort_unstable();
}
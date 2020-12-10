use std::fs::read_to_string;

fn main() {
	let input = read_to_string("d10-input").expect("something went wrong reading file");
	let mut data: Vec<usize> = input.lines().map(|n| n.parse().unwrap()).collect();
	data.push(0);
	data.sort_unstable();

	let mut paths: Vec<usize> = vec![0; *data.last().unwrap() + 3];
	paths[*data.last().unwrap()] = 1;
	for n in data.iter().rev().skip(1) {
		paths[*n] += paths[n + 1] + paths[n + 2] + paths[n + 3];
	}
	println!("Number of distinct paths: {}", paths[0]);
}
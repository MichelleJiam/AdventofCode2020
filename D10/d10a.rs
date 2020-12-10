// use std::io::Read;
use std::fs::read_to_string;

fn main() {
	let input = read_to_string("d10-input").expect("something went wrong reading file");
	let mut data: Vec<i32> = input.lines().map(|n| n.parse().unwrap()).collect();
	data.push(0); // outlet
	data.sort();
	let mut diff: Vec<_> = data.windows(2).map(|n| n[1]-n[0]).collect();
	let ones = diff.iter().filter(|&n| *n == 1).count();
	let twos = diff.iter().filter(|&n| *n == 2).count();
	let threes = diff.iter().filter(|&n| *n == 3).count() + 1; // device adapter diff
	println!("Number of 1s {}, 2s {}, 3s {}", ones, twos, threes);
	println!("Answer to part 1: {} x {} = {}", ones, threes, ones * threes);
}
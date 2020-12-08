use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("d06-input").expect("file not found");
	let mut input = String::new();
	file.read_to_string(&mut input).expect("something went wrong reading file");
	
	let mut sum = 0;
	let data: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();
	for line in data.iter() {
		let mut group: Vec<char> = line.chars().filter(|&c| !c.is_whitespace()).collect();
		group.sort();
		group.dedup();
		sum += group.len();
	}
	println!("Sum of answers: {}", sum);
}
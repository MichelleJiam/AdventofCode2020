use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("d06-input").expect("file not found");
	let mut input = String::new();
	file.read_to_string(&mut input).expect("something went wrong reading file");
	
	let mut sum = 0;
	let data: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();
	for line in data.iter() {
		let group_size = line.chars().filter(|&c| c == '\n').count() + 1;
		let mut group: Vec<char> = line.chars().filter(|&c| !c.is_whitespace()).collect();
		group.sort();
		let mut answers = group.clone();
		answers.dedup();
		for answer in answers.iter() {
			let count = group.iter().filter(|&c| c == answer).count();
			if count == group_size {
				sum += 1;
			}
		}
	}
	println!("Sum of answers: {}", sum);
}
use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("d02-input").unwrap();
	let mut input = String::new();
	file.read_to_string(&mut input).unwrap();

	let mut valid_pass = 0;
	for line in input.lines() {
		let line2 = &line.replace(":", " ").replace("-", " ");
		let chunks: Vec<_> = line2.split_whitespace().collect();
		// chunks[0] = first pos, chunks[1] = second pos, chunks[2] = char, chunks[3] = password
		let pos_1 = chunks[0].parse::<usize>().unwrap();
		let pos_2 = chunks[1].parse::<usize>().unwrap();
		let c1 = chunks[3].chars().nth(pos_1 - 1);
		let c2 = chunks[3].chars().nth(pos_2 - 1);
		let letter = chunks[2].parse::<char>().unwrap();
		if (c1.unwrap() == letter) ^ (c2.unwrap() == letter) {
			valid_pass += 1;
		}
	}
	println!("Number of valid passwords: {}", valid_pass);
}
use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("d02-input").unwrap();
	let mut input = String::new();
	file.read_to_string(&mut input).unwrap();

	let mut valid_pass = 0;
	for line in input.lines() {
		let line2 = &line.replace(":", " ");
		let chunks: Vec<_> = line2.split_whitespace().collect();
		// chunks[0] = range, chunks[1] = char, chunks[2] = password
		let bounds: Vec<_> = chunks[0].split("-").collect();
		// bounds[0] = lower limit, bounds[1] = upper limit
		let count = chunks[2].matches(chunks[1]).count();
		if bounds[0].parse::<usize>().unwrap() <= count {
			if count <= bounds[1].parse::<usize>().unwrap() {
				valid_pass += 1;
			}
		}
	}
	println!("Number of valid passwords: {}", valid_pass);
}
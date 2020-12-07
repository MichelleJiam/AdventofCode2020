use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("d04-input").unwrap();
	let mut input = String::new();
	file.read_to_string(&mut input).unwrap();

	let mut valid = 0u32;
	let data: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();
	for line in data.iter() {
		valid += check_validity(line);
	}
	println!("Number of valid passports: {}", valid);
}

fn check_validity(line: &str) -> u32 {
	let strings: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

	if strings.iter().all(|field| line.contains(field)) {
		return 1;
	} else {
		return 0;
	}
}
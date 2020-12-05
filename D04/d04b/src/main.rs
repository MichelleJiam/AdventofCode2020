extern crate regex;
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() {
	let mut file = File::open("./../d04-input").expect("file not found");
	let mut input = String::new();
	file.read_to_string(&mut input).expect("something went wrong reading file");

	let mut valid = 0u32;
	let mut data = Vec::<String>::new();
	for (index, line) in input.lines().enumerate() {
		if !line.is_empty() {
			let mut data2: Vec<String> = line.split_whitespace().map(str::to_string).collect();
			data.append(&mut data2);
		}
		if (line.is_empty() && !data.is_empty()) || index == input.lines().count() - 1 {
			valid += check_validity(&data);
			data.clear();
		}
	}
	println!("Number of valid passports: {}", valid);
}

fn check_validity(data: &Vec<String>) -> u32 {
	let strings: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
	let patterns: [&str; 7] = [
		"byr:19[2-9][0-9]|200[0-2]\\b",
		"iyr:201[0-9]|2020\\b",
		"eyr:202[0-9]|2030\\b",
		"hgt:(1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)\\b",
		"hcl:#[0-9a-f]{6}\\b",
		"ecl:amb|blu|brn|gry|grn|hzl|oth\\b",
		"pid:[0-9]{9}\\b",
	];
	let mut valid = 0;
	
	for (pos, string) in strings.iter().enumerate() {
		for entry in data.iter() {
			if entry.contains(string) {
				let pattern = Regex::new(patterns[pos]).unwrap();
				if pattern.is_match(entry) {
					valid += 1;
				}
			}
		}
	}
	match valid {
		7 => return 1,
		_ => return 0,
	}
}
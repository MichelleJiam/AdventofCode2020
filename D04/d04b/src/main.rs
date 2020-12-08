extern crate regex;
use std::fs::File;
use std::io::Read;
use regex::RegexSet;

fn main() {
	let mut file = File::open("d04-input").expect("file not found");
	let mut input = String::new();
	file.read_to_string(&mut input).expect("something went wrong reading file");

	let mut valid = 0u32;
	let data: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();
	for line in data.iter() {
		valid += check_validity(line);
	}
	println!("Number of valid passports: {}", valid);
}

fn check_validity(line: &str) -> u32 {
	let set = RegexSet::new(&[
		r"byr:19[2-9][0-9]|200[0-2]\b",
		r"iyr:201[0-9]|2020\b",
		r"eyr:202[0-9]|2030\b",
		r"hgt:(1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)\b",
		r"hcl:#[0-9a-f]{6}\b",
		r"ecl:amb|blu|brn|gry|grn|hzl|oth\b",
		r"pid:[0-9]{9}\b",
	]).unwrap();

	let matches: Vec<_> = set.matches(line).into_iter().collect();
	if matches.len() == 7 {
		return 1;
	} else {
		return 0;
	}
}
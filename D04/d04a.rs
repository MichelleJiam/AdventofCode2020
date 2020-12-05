use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("d04-input").unwrap();
	let mut input = String::new();
	file.read_to_string(&mut input).unwrap();

	let mut valid = 0u32;
	let mut data = Vec::<String>::new();
	for (index, line) in input.lines().enumerate() {
		if !line.is_empty() {
			let line2 = &line.replace(":", " ");
			let mut data2: Vec<String> = line2.split_whitespace().map(str::to_string).collect();
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
	let mut found = 0;
	let strings: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

	for string in strings.iter() {
		if data.iter().any(|s| s == string) {
			found += 1;
		}
	}
	match found {
		7 => return 1,
		_ => return 0,
	}
}
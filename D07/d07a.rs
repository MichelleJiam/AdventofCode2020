use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("d07-input").expect("file not found");
	let mut input = String::new();
	file.read_to_string(&mut input).expect("something went wrong reading file");

	let data: Vec<String> = input.split("\n").map(|s| s.to_string()).collect();
	let mut types: Vec<String> = vec![" shiny gold".to_string()];
    for (index, _line) in data.iter().enumerate() {
		find_bags(&mut types, index, &data);
	}
	println!("Types of bags: {}", types.len() - 1);
}

fn find_bags(types: &mut Vec<String>, index: usize, data: &Vec<String>) -> () {
	for line in data.iter() {
		if types.get(index) != None && line.contains(types.get(index).unwrap()) {
			let pos = line.find(" bags").unwrap();
			let str = " ".to_string() + &line[0..pos].to_string();
			if !types.contains(&str) {
				types.push(str);
			}
		}
	}
}
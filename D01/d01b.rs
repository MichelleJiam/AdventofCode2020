use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("input").unwrap();
	let mut numbers = String::new();
	file.read_to_string(&mut numbers).unwrap();
	let mut vector: Vec<i32> = vec![];

	for line in numbers.lines() {
		vector.push(line.parse::<i32>().unwrap());
	}
	for i in vector.iter() {
		for j in vector.iter() {
			for k in vector.iter() {
				if i + j + k == 2020 {
					println!("{} * {} * {} = {}", i, j, k, i * j * k);
					return();
				}
			}
		}
	}
	println!("No possible combination found");
}
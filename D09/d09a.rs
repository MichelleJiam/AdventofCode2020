use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("d09-input").expect("file not found");
	let mut input = String::new();
	file.read_to_string(&mut input).expect("something went wrong reading file");

	let data: Vec<u128> = input.split("\n").map(|s| s.parse::<u128>().unwrap()).collect();
	for (index, num) in data.iter().enumerate() {
		if index < 25 {
			continue;
		}
		if !can_sum(*num, &data[index - 25..index]) {
			println!("{} cannot be summed", num);
			break;
		}
	}
}

fn can_sum(num: u128, subset: &[u128]) -> bool {
	for n1 in subset.iter() {
		for n2 in subset.iter() {
			if n1 != n2 && n1 + n2 == num {
				return true;
			}
		}
	}
	return false;
}
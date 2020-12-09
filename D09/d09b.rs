use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("d09-input").expect("file not found");
	let mut input = String::new();
	file.read_to_string(&mut input).expect("something went wrong reading file");

	let data: Vec<u128> = input.split("\n").map(|s| s.parse::<u128>().unwrap()).collect();
	let len = data.len();
	let target = 248131121;
	let (res, start, end) = find_sum_range(target, &data[..len], len);
	if res == true {
		println!("Smallest in sum range {}, largest {}, product = {}", start, end, start + end);
	} else {
		println!("No match found");
	}
}

fn find_sum_range(target: u128, data: &[u128], len: usize) -> (bool, u128, u128) {
	let mut current_sum = 0;
	let (mut start, mut end) = (0, 0);
	while current_sum != target && end < len {
		if current_sum > target {
			current_sum -= data[start];
			start += 1;
		}
		else {
			current_sum += data[end];
			end += 1;
		}
	}
	if current_sum == target {
		let mut range: Vec<u128> = data[start..end].iter().cloned().collect();
		range.sort();
		return (true, range[0], *range.last().unwrap());
	}
	return (false, 0, 0);
}
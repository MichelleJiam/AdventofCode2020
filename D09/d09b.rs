use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("d09-input").expect("file not found");
	let mut input = String::new();
	file.read_to_string(&mut input).expect("something went wrong reading file");

	let target = 248131121;
	let mut data: Vec<u128> = input.split("\n").map(|s| s.parse::<u128>().unwrap()).collect();
	let pos = data.iter().position(|&x| x == target).unwrap();
	if !data.iter().skip(pos).any(|&x| x < target) {
		data.truncate(pos);
	}
	let len = data.len();
	let (res, n1, n2) = find_sum_range(target, &data[..len], len);
	if res == true {
		println!("Smallest in sum range {}, largest {}, product = {}", n1, n2, n1 + n2);
	} else {
		println!("No match found");
	}
}

fn find_sum_range(target: u128, data: &[u128], len: usize) -> (bool, u128, u128) {
	let mut i = 0;
	while i < len {
		let n1 = data[i];
		let mut nums: Vec<u128> = vec![n1];
		for (j, n2) in data.iter().skip(i + 1).enumerate() {
			nums.push(*n2);
			if *n2 > target {
				i = j;
				break;
			} else if nums.iter().sum::<u128>() > target {
				break;
			} else if nums.iter().sum::<u128>() == target {
				nums.sort();
				return (true, nums[0], *nums.last().unwrap());
			}
		}
		i += 1;
	}
	return (false, 0, 0);
}
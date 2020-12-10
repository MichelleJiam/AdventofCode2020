use std::fs::read_to_string;

fn main() {
	let input = read_to_string("d10-input").expect("something went wrong reading file");
	let mut data: Vec<i32> = input.lines().map(|n| n.parse().unwrap()).collect();
	
	data.sort_unstable();
	let (ones, twos, threes) = data
		.windows(2)
		.fold((1, 0, 1), |(ones, twos, threes), n| match n[1] - n[0] {
			1 => (ones + 1, twos, threes),
			2 => (ones, twos + 1, threes),
			3 => (ones, twos, threes + 1),
			_ => unreachable!(),
	});
	println!("Number of 1s {}, 2s {}, 3s {}", ones, twos, threes);
	println!("Answer to part 1: {} x {} = {}", ones, threes, ones * threes);
}
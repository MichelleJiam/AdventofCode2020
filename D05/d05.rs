use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("d05-input").expect("file not found");
	let mut input = String::new();
	file.read_to_string(&mut input).expect("something went wrong reading file");

	let mut ids: Vec<u32> = vec![];
	for line in input.lines() {
		let mut row = (0.0f32, 127.0f32);
		let mut col = (0.0f32, 7.0f32);

		for i in line.chars() {
			match i {
				'F' => row.1 = (row.1 - ((row.1 - row.0) / 2.0)).floor(),
				'B' => row.0 = (row.0 + ((row.1 - row.0) / 2.0)).ceil(),
				'L' => col.1 = (col.1 - ((col.1 - col.0) / 2.0)).floor(),
				'R' => col.0 = (col.0 + ((col.1 - col.0) / 2.0)).ceil(),
				_ => panic!("Invalid code"),
			}
		}
		let id = (row.0 * 8.0 + col.0) as u32;
		ids.push(id);
	}
	ids.sort();
	println!("Highest ID found: {}, lowest: {}", ids[ids.len() - 1], ids[0]);
	for (count, i) in ids.iter().enumerate() {
		let mut j = 0;
		if count + 1 < ids.len(){
			j = ids[count + 1];
		}
		if j != 0 && &j - i > 1 {
			println!("Missing seat between {} & {}", i, j);
		}
	}
}
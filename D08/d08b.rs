use std::fs::File;
use std::io::Read;

fn main() {
	let mut file = File::open("d08-input").expect("file not found");
	let mut input = String::new();
	file.read_to_string(&mut input).expect("something went wrong reading file");

	let mut data: Vec<String> = input.split("\n").map(|s| s.to_string()).collect();
	for (index, line) in input.lines().enumerate() {
		if line.contains("acc") {
			continue;
		}
		swap(&mut data, index);
		if let Some(acc) = try_run(&mut data.clone()) {
			println!("acc is {}", acc);
			break;
		}
		swap(&mut data, index);
	}
}

fn	swap(data: &mut Vec<String>, index: usize) -> () {
	data[index] = match &data[index][..3] {
		"jmp" => data[index].replace("jmp", "nop"),
		"nop" => data[index].replace("nop", "jmp"),
		_ => data[index].to_string(),
	};
}

fn try_run(data: &mut Vec<String>) -> Option<i32> {
	let (mut acc, mut pos) = (0i32, 0i32);
	let mut visited: Vec<bool> = vec![false; data.len()];
	while pos >= 0 && pos < data.len() as i32 {
		let line = data.iter().cloned().nth(pos as usize).unwrap();
		if visited[pos as usize] {
			break;
		}
		visited[pos as usize] = true;
		let amount = line[4..].parse::<i32>().unwrap();
		match &line[..3] {
			"acc" => acc += amount,
			"jmp" => pos += amount - 1,
			_ => {}
		}
		pos += 1;
	}
	if pos < data.len() as i32 {
		None
	} else {
        Some(acc)
    }
}
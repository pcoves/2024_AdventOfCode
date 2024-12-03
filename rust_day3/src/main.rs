use std::fs::read_to_string;
use regex::Regex;

fn main() {
	//let binding = read_to_string("files/test.txt").unwrap();
	let binding = read_to_string("files/input.txt").unwrap();
	

	let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
	let input:i32 = re
		.captures_iter(&binding)
		.map(|mul| {
				mul.get(1).unwrap().as_str().parse::<i32>().unwrap() * mul.get(2).unwrap().as_str().parse::<i32>().unwrap()
		})
		.collect::<Vec<i32>>()
		.iter()
		.sum();

	println!("part1 total: {}", input);
}
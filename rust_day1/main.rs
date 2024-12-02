use std::fs::read_to_string;
use std::iter::zip;


fn main() {
	let binding = read_to_string("input.txt")
					.unwrap();

	let mut list0 = binding
				.lines()
				.clone()
        		.map(|s| s.split_whitespace().nth(0).unwrap().parse::<i32>().unwrap())
				.collect::<Vec<i32>>();
	list0.sort();
	let mut list1 = binding
				.lines()
				.clone()
				.map(|s| s.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap())
				.collect::<Vec<i32>>();
	list1.sort();

	let mut total = 0;
	for it in zip(list0, list1) {
		let (number0, number1) = it;
		let diff = (number0 - number1).abs();
		total += diff;
		println!("{} - {} : diff = {}", number0, number1, diff);
	}
	println!("Total = {}", total);
}
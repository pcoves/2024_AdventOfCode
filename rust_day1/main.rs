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

	step1(&list0, &list1);
	step2(&list0, &list1);
}

fn step1(list0: &Vec<i32>, list1 : &Vec<i32>) {
	let mut total = 0;
	for it in zip(list0, list1) {
		let (number0, number1) = it;
		let diff = (number0 - number1).abs();
		total += diff;
	}
	println!("Step 1 Total = {}", total);
}

fn step2(list0: &Vec<i32>, list1 : &Vec<i32>) {
	let mut total = 0;
	for number in list0 {
		let uccurence = list1.iter().filter(|&x| x == number).count() as i32;
		let similarity = number * uccurence;
		total += similarity;
	}
	println!("Step 2 Total = {}", total);
}
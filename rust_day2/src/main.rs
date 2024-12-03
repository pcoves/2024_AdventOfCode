use std::fs::read_to_string;

fn main() {
	//let binding = read_to_string("files/test.txt").unwrap();
	let binding = read_to_string("files/input.txt").unwrap();
	
	let report_list = binding
				.lines()
        		.map(|s| s.split_whitespace()
							.map(|number| number.parse::<i32>()
												.unwrap())
							.collect::<Vec<i32>>())
				.collect::<Vec<Vec<i32>>>();

	step1(&report_list)
}

fn step1(list: &Vec<Vec<i32>>) {
	let total: i32 = list.iter().map(|report| match is_safe(report){
		true => 1,
		false => 0,
	}).sum();
	println!("Step 1 Total = {}", total);
}

fn is_safe(report: &Vec<i32>) -> bool {
	if report.len() <= 0
		{return false;}
	else {
		let sign = (report[0] - report[1]).signum();
		return report.windows(2).all(|w| {(w[0] - w[1]).signum() == sign && 1 <= (w[0] - w[1]).abs() && (w[0] - w[1]).abs() <= 3});
	}
}
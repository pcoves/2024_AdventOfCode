use std::fs::read_to_string;

fn main() {
    //let binding = read_to_string("files/test.txt").unwrap();
    let binding = read_to_string("files/input.txt").unwrap();

    let report_list = binding
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    step1(&report_list);
    step2(&report_list);
}

fn step1(list: &[Vec<i32>]) {
    let total: i32 = list
        .iter()
        .map(|report| match is_safe(report) {
            true => 1,
            false => 0,
        })
        .sum();
    println!("Step 1 Total = {}", total);
}

fn is_safe(report: &[i32]) -> bool {
	let sign = (report[0] - report[1]).signum();
	report.windows(2).all(|w| {
		(w[0] - w[1]).signum() == sign && 1 <= (w[0] - w[1]).abs() && (w[0] - w[1]).abs() <= 3
	})
}

fn step2(list: &[Vec<i32>]) {
    let total: i32 = list
        .iter()
        .map(|report| match is_safe(report) {
            true => 1,
            false => {
                for (index, _element) in report.iter().enumerate() {
                    if is_safe(&remove_copy(report, index)) {
                        return 1
                    }
                }
                0
            }
        })
        .sum();
    println!("Step 2 Total = {}", total);
}

fn remove_copy(list: &[i32], index: usize) -> Vec<i32> {
    let mut return_list = list.to_owned();
    return_list.remove(index);
    return_list
}

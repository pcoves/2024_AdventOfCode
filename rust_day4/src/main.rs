use std::fs::read_to_string;

fn main() {
	//let binding = read_to_string("files/test.txt").unwrap();
	let binding = read_to_string("files/input.txt").unwrap();

	let input: Vec<&str> = binding.lines().collect();

	step1(input.clone());
	step2(input.clone());
}

fn get_columns(list: Vec<&str>) -> Vec<String> {
	(0..list[0].len())
		.map(|col| {
			list
				.iter()
				.map(|row| row.chars().nth(col).unwrap())
				.collect()
		})
		.collect()
}

fn get_diagonals(list: Vec<&str>) -> (Vec<String>, Vec<String>) {
	let mut main_diagonals = vec![String::new(); list.len() + list[0].len() - 1];
    let mut anti_diagonals = vec![String::new(); list.len() + list[0].len() - 1];

    for row in 0..list.len() {
        for col in 0..list[0].len() {
            let c = list[row].chars().nth(col).unwrap();
            main_diagonals[list[0].len() - 1 + row - col].push(c);
            anti_diagonals[row + col].push(c);
        }
    }
	return (main_diagonals, anti_diagonals)
}

fn count_xmas(line: &str) -> i32 {
	return (line.matches("XMAS").count() 
							+ line.matches("SAMX").count()) as i32;
}

fn step1(input: Vec<&str>) {
	let mut total:i32 = input
		.iter()
		.map(|line| count_xmas(line))
		.sum::<i32>();
	let reverse_input = get_columns(input.clone());
	total += reverse_input
		.iter()
		.map(|line| count_xmas(line))
		.sum::<i32>();
	let (main_diagonals, anti_diagonals) = get_diagonals(input.clone());
	total += main_diagonals
		.iter()
		.map(|line| count_xmas(line))
		.sum::<i32>();
	total += anti_diagonals
		.iter()
		.map(|line| count_xmas(line))
		.sum::<i32>();
	println!("step1 total : {}", total);
}
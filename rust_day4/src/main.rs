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

fn step2(input: Vec<&str>) {
	let mut total:i32 = 0;
    for row in 1..input.len()-1 {
        for col in 1..input[0].len()-1 {
			if 'A' == input[row].chars().nth(col).unwrap() {
				let square = [&input[row-1][col-1..col+2], &input[row][col-1..col+2], &input[row+1][col-1..col+2]];
				total += test_all_x_mas(&(square.join(""))) as i32;
			}
		}
	}
	println!("step2 total : {}", total);
}

fn rotate_square(square: &str) -> [char; 9] {
	return [
		square.chars().nth(2).unwrap(), square.chars().nth(5).unwrap(), square.chars().nth(8).unwrap(),
		square.chars().nth(1).unwrap(), square.chars().nth(4).unwrap(), square.chars().nth(7).unwrap(),
		square.chars().nth(0).unwrap(), square.chars().nth(3).unwrap(), square.chars().nth(6).unwrap(),];
}

fn test_x_mas(square: &str) -> bool {
	if 'M' == square.chars().nth(0).unwrap() 
	&& 'M' == square.chars().nth(6).unwrap() 
	&& 'S' == square.chars().nth(2).unwrap() 
	&& 'S' == square.chars().nth(8).unwrap(){
		return true;
	}
	return false;
}

fn test_all_x_mas(square: &str) -> bool {
	let mut rotated_square = square.to_string();
	for _i in 0..4 {
		if test_x_mas(&rotated_square) {
			return true;
		} else {
			rotated_square = rotate_square(&rotated_square).iter().collect::<String>();
		}
	}
	return false;
}

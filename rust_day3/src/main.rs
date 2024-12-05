use regex::Regex;
use std::fs::read_to_string;

fn main() {
    //let binding = read_to_string("files/test.txt").unwrap();
    let binding = read_to_string("files/input.txt").unwrap();

    let re_do = Regex::new(r"(?s)don't\(\).*?(do\(\)|$)").unwrap();
    let input = re_do.replace_all(&binding, "");

    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result: i32 = re_mul
        .captures_iter(&input)
        .map(|mul| {
            mul.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * mul.get(2).unwrap().as_str().parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    println!("part1 total: {}", result);
}

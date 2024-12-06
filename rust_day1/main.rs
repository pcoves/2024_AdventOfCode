use std::iter::zip;
use std::str::FromStr;

fn main() {
    let binding = include_str!("input.txt");

    let (mut list0, mut list1) =
        binding
            .lines()
            .fold((vec![], vec![]), |(mut lhs, mut rhs), line| {
                let mut words = line.split_whitespace();
                lhs.push(words.next().map(i32::from_str).unwrap().unwrap());
                rhs.push(words.next().unwrap().parse::<i32>().unwrap());
                (lhs, rhs)
            });
    list0.sort();
    list1.sort();

    step1(&list0, &list1);
    step2(&list0, &list1);
}

fn step1(list0: &Vec<i32>, list1: &Vec<i32>) {
    let total = zip(list0, list1).fold(0, |acc, (lhs, rhs)| acc + (lhs - rhs).abs());
    println!("Step 1 Total = {}", total);
}

fn step2(list0: &Vec<i32>, list1: &[i32]) {
    let mut total = 0;
    for number in list0 {
        let uccurence = list1.iter().filter(|&x| x == number).count() as i32;
        let similarity = number * uccurence;
        total += similarity;
    }
    println!("Step 2 Total = {}", total);
}

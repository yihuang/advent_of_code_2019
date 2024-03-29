use std::iter::successors;

fn parse_input() -> Vec<u64> {
    let input: &str = include_str!("day1.input");
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1() {
    let amount: u64 = parse_input().into_iter().map(|n| n / 3 - 2).sum();
    println!("{}", amount);
}

fn fuel(n: u64) -> u64 {
    successors(Some(n), |n| (n / 3).checked_sub(2))
        .skip(1)
        .sum()
}

pub fn part2() {
    let amount: u64 = parse_input().into_iter().map(fuel).sum();
    println!("{}", amount);
}

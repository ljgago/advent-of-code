//! # Advent of Code - Day 2

mod part1;
mod part2;

fn main() {
    let input = include_str!("../puzzle_2024_02.txt");

    println!("--- Part One ---");
    println!("Result: {}", part1::part1(input));

    println!("--- Part Two ---");
    println!("Result: {}", part2::part2(input));
}

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .trim_matches('\n')
        .lines()
        .map(|line| line.trim().split(" ").collect::<Vec<&str>>())
        .map(|items| {
            items
                .into_iter()
                .map(|item| item.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

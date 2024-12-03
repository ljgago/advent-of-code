//! # Advent of Code - Day 1

mod part1;
mod part2;

fn main() {
    let input = include_str!("../puzzle_2024_01.txt");

    println!("--- Part One ---");
    println!("Result: {}", part1::part1(input));

    println!("--- Part Two ---");
    println!("Result: {}", part2::part2(input));
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .trim()
        .trim_matches('\n')
        .lines()
        .map(|line| line.trim().split("   ").collect::<Vec<&str>>())
        .map(|item| {
            (
                item[0].parse::<i32>().unwrap(),
                item[1].parse::<i32>().unwrap(),
            )
        })
        .unzip()
}

//! # Advent of Code - Day 10

mod part1;
mod part2;
mod parser;

fn main() {
    let input = include_str!("../puzzle_2024_10.txt");

    println!("--- Part One ---");
    println!("Result: {}", part1::part1(input));

    println!("--- Part Two ---");
    println!("Result: {}", part2::part2(input));
}

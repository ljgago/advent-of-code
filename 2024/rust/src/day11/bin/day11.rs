//! # Advent of Code - Day 11

mod part1;
mod part2;

fn main() {
    let input = include_str!("../puzzle_2024_11.txt");

    println!("--- Part One ---");
    println!("Result: {}", part1::part1(input, 25));

    println!("--- Part Two ---");
    println!("Result: {}", part2::part2(input, 75));
}

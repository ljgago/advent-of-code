//! # Advent of Code - Day 11 - Part Two

use crate::part1;

pub fn part2(input: &str, blinks: usize) -> usize {
    part1::part1(input, blinks)
}

#[cfg(test)]
mod day11 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "125 17";
        assert_eq!(part2(input, 25), 55312);
    }
}

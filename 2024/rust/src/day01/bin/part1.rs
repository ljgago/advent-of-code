//! # Advent of Code - Day 1 - Part One

use crate::parse;
use std::iter::zip;

pub fn part1(input: &str) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = parse(input);

    left.sort();
    right.sort();

    zip(left, right).map(|(l, r)| (l - r).abs()).sum()
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "#;
        assert_eq!(part1(input), 11);
    }
}

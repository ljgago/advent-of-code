//! # Advent of Code - Day 1 - Part Two

use crate::parse;
use std::collections::HashMap;

pub fn part2(input: &str) -> i32 {
    let (left, right): (Vec<i32>, Vec<i32>) = parse(input);
    let score: HashMap<i32, i32> = right.into_iter().fold(HashMap::new(), |mut score, x| {
        *score.entry(x).or_insert(0) += 1;
        score
    });

    left.iter()
        .map(|key| match score.get(&key) {
            Some(&val) => key * val,
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn test_part2() {
        let input = r#"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "#;
        assert_eq!(part2(input), 31);
    }
}

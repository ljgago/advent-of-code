//! # Advent of Code - Day 2 - Part One

use crate::parse;

pub fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|report| {
            let diff: Vec<i32> = report.windows(2).map(|pair| pair[0] - pair[1]).collect();

            let cond1 = diff.iter().all(|&x| x > 0 && x <= 3);
            let cond2 = diff.iter().all(|&x| x < 0 && x >= -3);

            match cond1 || cond2 {
                true => 1,
                false => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod day02 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "#;
        assert_eq!(part1(input), 2);
    }
}

//! # Advent of Code - Day 2 - Part Two

use crate::parse;

pub fn part2(input: &str) -> usize {
    let mut result: Vec<usize> = Vec::new();

    for report in parse(input) {
        for new_report in expand_report(report) {
            if check_report(&new_report) == "safe" {
                result.push(1);
                break;
            }
        }
    }

    result.iter().count()
}

fn check_report(report: &[i32]) -> &str {
    let diff: Vec<i32> = report.windows(2).map(|pair| pair[0] - pair[1]).collect();

    let cond1 = diff.iter().all(|&x| x > 0 && x <= 3);
    let cond2 = diff.iter().all(|&x| x < 0 && x >= -3);

    match cond1 || cond2 {
        true => "safe",
        false => "unsafe",
    }
}

fn expand_report(report: Vec<i32>) -> Vec<Vec<i32>> {
    let mut variations: Vec<Vec<i32>> = Vec::new();

    for index in 0..report.len() {
        let mut variation = report.clone();
        variation.remove(index);
        variations.push(variation);
    }

    variations
}

#[cfg(test)]
mod day02 {
    use super::*;

    #[test]
    fn test_part2() {
        let input = r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "#;
        assert_eq!(part2(input), 4);
    }
}

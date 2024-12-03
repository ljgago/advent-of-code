//! # Advent of Code - Day 3 - Part Two

use regex::Regex;

use crate::part1::solve_instructions;

pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(don't\(\))|(do\(\))").unwrap();
    let instructions: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    let mut enabled_instructions: Vec<&str> = Vec::new();
    let mut is_enabled = true;

    for inst in instructions {
        match inst {
            "don't()" => is_enabled = false,
            "do()" => is_enabled = true,
            _ => {
                if is_enabled {
                    enabled_instructions.push(inst);
                }
            }
        }
    }

    solve_instructions(enabled_instructions)
}

#[cfg(test)]
mod day03 {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), 48);
    }
}

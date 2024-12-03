//! # Advent of Code - Day 3 - Part One

use regex::Regex;

pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let instructions: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    solve_instructions(instructions)
}

pub fn solve_instructions(instructions: Vec<&str>) -> i32 {
    let mut result = 0;

    for inst in instructions {
        let inst = inst.trim_matches(['m', 'u', 'l', '(', ')']);

        match inst.split(",").collect::<Vec<&str>>()[..] {
            [x, y] => {
                let x = x.parse::<i32>().unwrap();
                let y = y.parse::<i32>().unwrap();

                result += x * y;
            }
            _ => continue
        }
    }

    result
}

#[cfg(test)]
mod day03 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), 161);
    }
}

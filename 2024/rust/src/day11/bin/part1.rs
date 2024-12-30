//! # Advent of Code - Day 11 - Part One

pub fn part1(input: &str, blinks: usize) -> usize {
    (0..blinks).fold(input.to_string(), |stones, _| {
        stones
            .clone()
            .split(" ")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|stone| {
                if stone == "0" {
                    return "1".to_string();
                }

                if has_even_digits(stone) {
                    return split_stone(stone);
                }

                return multiply_by_2024(stone);
            })
            .collect::<Vec<String>>()
            .join(" ")
    })
    .split(" ")
    .count()
}

fn has_even_digits(stone: &str) -> bool {
    stone.len() % 2 == 0
}

fn split_stone(stone: &str) -> String {
    let (left, right) = stone.split_at(stone.len() / 2);
    let right = right.parse::<usize>().unwrap().to_string();

    vec![left, &right].join(" ")
}

fn multiply_by_2024(stone: &str) -> String {
    let stone_as_number = stone.parse::<usize>().unwrap();

    (stone_as_number * 2024).to_string()
}

#[cfg(test)]
mod day11 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "125 17";
        assert_eq!(part1(input, 25), 55312);
    }
}

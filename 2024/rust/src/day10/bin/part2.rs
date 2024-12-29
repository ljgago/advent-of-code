//! # Advent of Code - Day 10 - Part Two

use std::collections::HashMap;

use crate::{parser, part1};

pub fn part2(input: &str) -> usize {
    let matrix = parser::parse(input);
    let (tree, zeros) = part1::generate_tree_zeros(matrix);
    let mut trailheads: Vec<usize> = Vec::new();

    for point in zeros {
        trailheads.push(get_trailhead_score(&tree, point, 0));
    }

    trailheads.iter().sum()
}

fn get_trailhead_score(
    tree: &HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>>,
    point: (usize, usize, usize),
    count: usize,
) -> usize {
    let mut new_count = count;

    if point.2 == 9 {
        return new_count + 1;
    }

    for p in tree.get(&point).unwrap() {
        new_count = get_trailhead_score(tree, *p, new_count);
    }

    return new_count;
}

#[cfg(test)]
mod day10 {
    use super::*;

    #[test]
    fn test_part2() {
        let input = r#"
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
        "#;
        assert_eq!(part2(input), 81);
    }
}

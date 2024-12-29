//! # Advent of Code - Day 10 - Part One

use std::collections::{HashMap, HashSet};

use crate::parser;

pub fn part1(input: &str) -> usize {
    let matrix = parser::parse(input);
    let (tree, zeros) = generate_tree_zeros(matrix);
    let mut trailheads: Vec<HashSet<(usize, usize, usize)>> = Vec::new();

    for point in zeros {
        trailheads.push(get_trailhead_score(&tree, point, HashSet::new()));
    }

    trailheads.iter().map(|x| x.len()).sum()
}

pub fn generate_tree_zeros(matrix: Vec<Vec<usize>>) -> (HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>>, Vec<(usize, usize, usize)>)  {
    let mut tree: HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>> = HashMap::new();
    let mut zeros: Vec<(usize, usize, usize)> = Vec::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let value = matrix[i][j];

            if !tree.contains_key(&(i, j, value)) {
                tree.insert((i, j, value), vec![]);
            }

            if value == 0 {
                zeros.push((i, j, value));
            }

            if i > 0 {
                let up = matrix[i - 1][j];
                let point = (i - 1, j, up);

                if !tree.contains_key(&point) {
                    tree.insert(point, vec![]);
                }

                if (up as i32 - value as i32) == 1 {
                    if let Some(v) = tree.get_mut(&(i, j, value)) {
                        (*v).push(point);
                    }
                }
            }

            if i < matrix.len() - 1 {
                let down = matrix[i + 1][j];
                let point = (i + 1, j, down);

                if !tree.contains_key(&point) {
                    tree.insert(point, vec![]);
                }

                if (down as i32 - value as i32) == 1 {
                    if let Some(v) = tree.get_mut(&(i, j, value)) {
                        v.push(point);
                    }
                }
            }

            if j > 0 {
                let left = matrix[i][j - 1];
                let point = (i, j - 1, left);

                if !tree.contains_key(&point) {
                    tree.insert(point, vec![]);
                }

                if (left as i32 - value as i32) == 1 {
                    if let Some(v) = tree.get_mut(&(i, j, value)) {
                        v.push(point);
                    }
                }
            }

            if j < matrix[0].len() - 1 {
                let right = matrix[i][j + 1];
                let point = (i, j + 1, right);

                if !tree.contains_key(&point) {
                    tree.insert(point, vec![]);
                }

                if (right as i32 - value as i32) == 1 {
                    if let Some(v) = tree.get_mut(&(i, j, value)) {
                        v.push(point);
                    }
                }
            }
        }
    }

    return (tree, zeros);
}

fn get_trailhead_score(
    tree: &HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>>,
    point: (usize, usize, usize),
    set_points: HashSet<(usize, usize, usize)>,
) -> HashSet<(usize, usize, usize)> {
    let mut new_set_points = set_points;

    if point.2 == 9 {
        new_set_points.insert(point);
        return new_set_points;
    }

    for p in tree.get(&point).unwrap() {
        let set_points_2 = get_trailhead_score(tree, *p, new_set_points.clone());
        new_set_points.extend(&set_points_2);
    }

    return new_set_points;
}

#[cfg(test)]
mod day10 {
    use super::*;

    #[test]
    fn test_part1() {
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
        assert_eq!(part1(input), 36);
    }
}

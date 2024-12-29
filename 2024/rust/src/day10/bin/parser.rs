//! # Advent of Code - Day 10 - Perser

pub fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .trim_matches(['\n', ' '])
        .lines()
        .map(|line| {
            line.trim_matches(['\n', ' '])
                .chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod day10 {
    use super::*;

    #[test]
    fn test_parse() {
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
        let expected: Vec<Vec<usize>> = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        assert_eq!(parse(input), expected);
    }
}

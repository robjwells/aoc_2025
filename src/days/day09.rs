#![allow(unused, dead_code, unused_mut)]

use itertools::Itertools;

use crate::util::Answer;
pub fn solve(input: &str) -> anyhow::Result<String> {
    let points = parse_input(input);
    Answer::first(9, solve_part_one(&points)).report()
}

fn solve_part_one(points: &[(u64, u64)]) -> u64 {
    points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
        .max()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod test {
    static TEST_INPUT: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn parse_test_input() {
        let expected = vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];
        let points = super::parse_input(TEST_INPUT);
        assert_eq!(points, expected);
    }

    #[test]
    pub fn part_one_test_input() {
        let points = super::parse_input(TEST_INPUT);
        let res = super::solve_part_one(&points);
        assert_eq!(res, 50);
    }

    #[test]
    pub fn part_one_known_answer() {
        let points = super::parse_input(crate::days::get_input(9).unwrap());
        let res = super::solve_part_one(&points);
        assert_eq!(res, 4758598740);
    }
}

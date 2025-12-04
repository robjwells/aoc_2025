use std::{collections::HashSet, ops::RangeInclusive};

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let mut grid = parse_input(input);
    let p1 = solve_part_one(&grid);
    let p2 = solve_part_two(&mut grid);
    Answer::first(4, p1).second(p2).report()
}

fn parse_input(s: &str) -> Grid {
    let mut filled = HashSet::with_capacity(100);
    let mut max_row_idx = 0;
    let mut max_col_idx = 0;
    for (row_idx, row) in s.lines().enumerate() {
        max_row_idx = max_row_idx.max(row_idx as i16);
        for (col_idx, col_char) in row.chars().enumerate() {
            max_col_idx = max_col_idx.max(col_idx as i16);
            if col_char == '@' {
                filled.insert((row_idx as i16, col_idx as i16));
            }
        }
    }
    Grid {
        row_range: 0..=max_row_idx,
        col_range: 0..=max_col_idx,
        filled,
    }
}

fn solve_part_one(grid: &Grid) -> usize {
    grid.accessible_rolls().len()
}

fn solve_part_two(grid: &mut Grid) -> usize {
    let mut total = 0;
    while let removed = grid.remove_accessible()
        && removed != 0
    {
        total += removed;
    }
    total
}

struct Grid {
    row_range: RangeInclusive<i16>,
    col_range: RangeInclusive<i16>,
    filled: HashSet<(i16, i16)>,
}

impl Grid {
    const NEIGHBOUR_DELTAS: [(i16, i16); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    #[allow(dead_code)]
    fn contains(&self, location: &(i16, i16)) -> bool {
        self.filled.contains(location)
    }

    #[allow(dead_code)]
    fn in_grid(&self, location: &(i16, i16)) -> bool {
        let (row, col) = location;
        self.row_range.contains(row) && self.col_range.contains(col)
    }

    fn filled_neighbours(&self, location: &(i16, i16)) -> Vec<(i16, i16)> {
        let (row, col) = location;
        Self::NEIGHBOUR_DELTAS
            .iter()
            .map(|&(rd, cd)| (row - rd, col - cd))
            .filter(|location| self.filled.contains(location))
            .collect()
    }

    fn accessible_rolls(&self) -> Vec<(i16, i16)> {
        let mut accessible = Vec::new();
        for roll in &self.filled {
            let filled_neighbours = self.filled_neighbours(roll);
            if filled_neighbours.len() < 4 {
                accessible.push(*roll);
            }
        }
        accessible
    }

    fn remove_accessible(&mut self) -> usize {
        let accessible = self.accessible_rolls();
        for roll in &accessible {
            self.filled.remove(roll);
        }
        accessible.len()
    }
}

#[cfg(test)]
mod test {
    use super::parse_input;

    static TEST_INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn parse_test_input() {
        let locations = parse_input(TEST_INPUT);
        assert!(locations.contains(&(0, 2)));
        assert!(locations.contains(&(0, 8)));
        assert!(locations.contains(&(1, 0)));
        assert!(locations.contains(&(1, 1)));
        assert!(locations.contains(&(1, 9)));
        assert!(locations.contains(&(9, 0)));
        assert!(locations.contains(&(9, 8)));
    }

    #[test]
    fn test_input_neighbours() {
        let grid = parse_input(TEST_INPUT);
        let mut neighbours = grid.filled_neighbours(&(4, 9));
        neighbours.sort();
        let mut expected = vec![(3, 8), (4, 8), (5, 9)];
        expected.sort();
        assert_eq!(neighbours, expected);
    }

    #[test]
    fn part_one_test_input() {
        let locations = parse_input(TEST_INPUT);
        let result = super::solve_part_one(&locations);
        assert_eq!(result, 13);
    }

    #[test]
    fn part_one_known_answer() {
        let locations = parse_input(crate::days::get_input(4).unwrap());
        let result = super::solve_part_one(&locations);
        assert_eq!(result, 1428);
    }

    #[test]
    fn part_two_test_input() {
        let mut locations = parse_input(TEST_INPUT);
        let result = super::solve_part_two(&mut locations);
        assert_eq!(result, 43);
    }

    #[test]
    fn part_two_known_answer() {
        let mut locations = parse_input(crate::days::get_input(4).unwrap());
        let result = super::solve_part_two(&mut locations);
        assert_eq!(result, 8936);
    }
}

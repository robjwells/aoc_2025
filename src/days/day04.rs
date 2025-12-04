use std::collections::HashSet;

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let grid = parse_input(input);
    let p1 = solve_part_one(&grid);
    Answer::first(4, p1).report()
}

fn parse_input(s: &str) -> Grid {
    let mut filled = HashSet::with_capacity(100);
    let mut max_row_idx = 0;
    let mut max_col_idx = 0;
    for (row_idx, row) in s.lines().enumerate() {
        max_row_idx = max_row_idx.max(row_idx);
        for (col_idx, col_char) in row.chars().enumerate() {
            max_col_idx = max_row_idx.max(row_idx);
            if col_char == '@' {
                filled.insert((row_idx, col_idx));
            }
        }
    }
    Grid {
        max_row_idx,
        max_col_idx,
        filled,
    }
}

fn solve_part_one(grid: &Grid) -> usize {
    let mut accessible = 0;
    for roll in &grid.filled {
        let filled_neighbours = grid.filled_neighbours(roll);
        if filled_neighbours.len() < 4 {
            accessible += 1;
        }
    }
    accessible
}

struct Grid {
    max_row_idx: usize,
    max_col_idx: usize,
    filled: HashSet<(usize, usize)>,
}

impl Grid {
    const NEIGHBOUR_DELTAS: [(isize, isize); 8] = [
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
    fn contains(&self, location: &(usize, usize)) -> bool {
        self.filled.contains(location)
    }

    #[allow(dead_code)]
    fn in_grid(&self, location: &(usize, usize)) -> bool {
        let (row, col) = location;
        (0..=self.max_row_idx).contains(row) && (0..=self.max_col_idx).contains(col)
    }

    fn filled_neighbours(&self, location: &(usize, usize)) -> HashSet<(usize, usize)> {
        let (row, col) = location;
        Self::NEIGHBOUR_DELTAS
            .iter()
            .map(|&(rd, cd)| (row.wrapping_add_signed(rd), col.wrapping_add_signed(cd)))
            .filter(|location| self.filled.contains(location))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

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
        let neighbours = grid.filled_neighbours(&(4, 9));
        let expected = HashSet::from([(3, 8), (4, 8), (5, 9)]);
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
}

use std::collections::{HashMap, HashSet};

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let mut grid = parse_input(input);
    let p1 = solve_part_one(&grid);
    let p2 = solve_part_two(&mut grid);
    Answer::first(4, p1).second(p2).report()
}

fn parse_input(s: &str) -> Grid {
    // Pad row with empty start and end columns, so idx ± 1 is always in bounds.
    let size = s.find('\n').unwrap() + 2;
    let mut present = HashSet::with_hasher(foldhash::fast::RandomState::default());
    let mut filled = vec![vec![false; size]; size];

    for (row_idx, row) in s.lines().enumerate() {
        for (col_idx, col_char) in row.chars().enumerate() {
            if col_char == '@' {
                filled[row_idx + 1][col_idx + 1] = true;
                present.insert((row_idx + 1, col_idx + 1));
            }
        }
    }

    Grid::new(present, filled)
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
    present: HashSet<(usize, usize), foldhash::fast::RandomState>,
    filled: Vec<Vec<bool>>,
    cache: HashMap<(usize, usize), Vec<(usize, usize)>, foldhash::fast::RandomState>,
}

impl Grid {
    fn new(
        present: HashSet<(usize, usize), foldhash::fast::RandomState>,
        filled: Vec<Vec<bool>>,
    ) -> Self {
        let mut cache = HashMap::with_capacity_and_hasher(
            present.len(),
            foldhash::fast::RandomState::default(),
        );
        for location in &present {
            let neighbours = Self::compute_neighbours(&filled, location);
            cache.insert(*location, neighbours);
        }
        Self {
            present,
            filled,
            cache,
        }
    }

    fn compute_neighbours(filled: &[Vec<bool>], location: &(usize, usize)) -> Vec<(usize, usize)> {
        let (row, col) = *location;
        let possible = [
            // Row above
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            // Same row
            (row, col - 1),
            (row, col + 1),
            // Row below
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ];
        let mut neighbours = Vec::with_capacity(8);
        for (row, col) in possible {
            if filled[row][col] {
                neighbours.push((row, col));
            }
        }
        neighbours
    }

    fn filled_neighbours(&self, location: &(usize, usize)) -> &[(usize, usize)] {
        self.cache.get(location).unwrap()
    }

    fn accessible_rolls(&self) -> Vec<(usize, usize)> {
        let mut accessible = Vec::new();
        for roll in &self.present {
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
            self.filled[roll.0][roll.1] = false;
            if self.present.remove(roll) {
                // Update the neighbour cache
                for neighbour in self.filled_neighbours(roll).to_vec() {
                    self.cache
                        .entry(neighbour)
                        .and_modify(|e| *e = Self::compute_neighbours(&self.filled, &neighbour));
                }
            }
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
        assert!(locations.filled[1][3]);
        assert!(locations.filled[1][9]);
        assert!(locations.filled[2][1]);
        assert!(locations.filled[2][2]);
        assert!(locations.filled[2][10]);
        assert!(locations.filled[10][1]);
        assert!(locations.filled[10][9]);
    }

    #[test]
    fn test_input_neighbours() {
        let grid = parse_input(TEST_INPUT);
        let mut neighbours = grid.filled_neighbours(&(5, 10)).to_vec();
        neighbours.sort();
        let mut expected = vec![(4, 9), (5, 9), (6, 10)];
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

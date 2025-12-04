use std::collections::VecDeque;

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
    let mut filled = vec![vec![false; size]; size];
    let mut to_check = VecDeque::new();

    for (row_idx, row) in s.lines().enumerate() {
        for (col_idx, col_char) in row.chars().enumerate() {
            if col_char == '@' {
                // +1 to both indices to account for the padding.
                filled[row_idx + 1][col_idx + 1] = true;
                to_check.push_back((row_idx + 1, col_idx + 1));
            }
        }
    }

    Grid::new(filled, to_check)
}

fn solve_part_one(grid: &Grid) -> usize {
    grid.pending_removal.len()
}

fn solve_part_two(grid: &mut Grid) -> usize {
    grid.remove_accessible()
}

struct Grid {
    filled: Vec<Vec<bool>>,
    pending_removal: VecDeque<(usize, usize)>,
    queued_for_removal: Vec<Vec<bool>>,
}

impl Grid {
    fn new(filled: Vec<Vec<bool>>, to_check: VecDeque<(usize, usize)>) -> Self {
        let size = filled.len();
        let mut grid = Self {
            filled,
            // In part 2 this grows up to 1819; 2048 * (8 * 2) == 32K.
            pending_removal: VecDeque::with_capacity(2048),
            queued_for_removal: vec![vec![false; size]; size],
        };
        // Find all the initially accessible rolls.
        for location in to_check {
            if grid.is_accessible(location) {
                grid.queue_for_removal(location);
            }
        }
        grid
    }

    fn filled_neighbours(&self, location: &(usize, usize)) -> Vec<(usize, usize)> {
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
        // There is a noticeable slow down here with .iter().filter().collect(), 8ms average.
        let mut neighbours = Vec::with_capacity(8);
        for (row, col) in possible {
            if self.filled[row][col] {
                neighbours.push((row, col));
            }
        }
        neighbours
    }

    fn already_queued(&self, location: (usize, usize)) -> bool {
        let (row, col) = location;
        self.queued_for_removal[row][col]
    }

    fn queue_for_removal(&mut self, location: (usize, usize)) {
        self.pending_removal.push_back(location);
        self.queued_for_removal[location.0][location.1] = true;
    }

    fn is_accessible(&self, location: (usize, usize)) -> bool {
        self.filled_neighbours(&location).len() < 4
    }

    fn remove_accessible(&mut self) -> usize {
        let mut removed = 0;
        while let Some(roll) = self.pending_removal.pop_front() {
            // Prevent double-counting removals (a location may have been removed earlier if it was
            // already present in the VecDeque). This will happen a few thousand times!
            self.filled[roll.0][roll.1] = false;
            removed += 1;
            // Maybe some neighbours can now be removed.
            for neighbour in self.filled_neighbours(&roll) {
                // Check we haven't already put this neighbour in the queue.
                if !self.already_queued(neighbour) && self.is_accessible(neighbour) {
                    self.queue_for_removal(neighbour);
                }
            }
        }
        removed
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

use std::collections::BTreeSet;

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let grid = parse_input(input);
    let p1 = grid.run_regular_split();
    Answer::first(7, p1).report()
}

fn parse_input(input: &str) -> Grid {
    let mut lines = input.lines();
    let start_column = lines.next().unwrap().find('S').unwrap();
    let mut splitters = BTreeSet::new();
    let mut max_row = 0;
    for (ri, row) in lines.enumerate() {
        let row_idx = ri + 1;
        max_row = max_row.max(row_idx);
        for (col_idx, col_char) in row.chars().enumerate() {
            if col_char == '^' {
                splitters.insert((row_idx, col_idx));
            }
        }
    }
    Grid {
        splitters,
        start_column,
        max_row,
    }
}

struct Grid {
    splitters: BTreeSet<(usize, usize)>,
    start_column: usize,
    max_row: usize,
}

impl Grid {
    fn run_regular_split(&self) -> usize {
        let mut beam_columns = BTreeSet::from([self.start_column]);
        let mut times_split = 0;
        for current_row in 1..=self.max_row {
            for &(_, column) in self
                .splitters
                .range((current_row, 0)..(current_row, usize::MAX))
            {
                if beam_columns.remove(&column) {
                    beam_columns.insert(column - 1);
                    beam_columns.insert(column + 1);
                    times_split += 1;
                }
            }
        }
        times_split
    }
}

#[cfg(test)]
mod test {
    static TEST_INPUT: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn parse_test_input() {
        let grid = super::parse_input(TEST_INPUT);
        assert_eq!(grid.start_column, 7);
        assert!(grid.splitters.contains(&(4, 6)));
        assert_eq!(grid.splitters.len(), 22);
        assert_eq!(grid.max_row, 15);
    }

    #[test]
    pub fn part_one_test_input() {
        let grid = super::parse_input(TEST_INPUT);
        let times_split = grid.run_regular_split();
        assert_eq!(times_split, 21);
    }

    #[test]
    pub fn part_one_known_answer() {
        let grid = super::parse_input(crate::days::get_input(7).unwrap());
        let times_split = grid.run_regular_split();
        assert_eq!(times_split, 1507);
    }
}

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let banks = parse_input(input);
    let p1 = solve_banks(2, &banks);
    let p2 = solve_banks(12, &banks);
    Answer::first(3, p1).second(p2).report()
}

fn parse_input(s: &str) -> Vec<Vec<u8>> {
    s.trim()
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect()
}

/// Sum the maximum n-digit joltage per bank.
fn solve_banks(n_batteries: usize, banks: &[Vec<u8>]) -> u64 {
    banks.iter().map(|b| max_for_bank(n_batteries, b)).sum()
}

fn slice_earliest_max(s: &[u8]) -> (usize, &u8) {
    s.iter()
        .enumerate()
        // idx subtracted from usize::MAX to prioritise earlier digits.
        .max_by_key(|(idx, value)| (*value, usize::MAX - idx))
        .expect("Tried to find max digit in empty slice")
}

/// Finds the maximum n-digit joltage for a given bank.
///
/// Repeatedly Chooses the maximum digit from the largest view into the bank that
/// allows for the rest of the n-digit number to be completed.
fn max_for_bank(n_batteries: usize, mut bank: &[u8]) -> u64 {
    let mut result = 0;
    for batteries_remaining in (0..n_batteries).rev() {
        let (idx, digit) = slice_earliest_max(&bank[..bank.len() - batteries_remaining]);
        result = result * 10 + *digit as u64;
        bank = &bank[idx + 1..];
    }
    result
}

#[cfg(test)]
mod test {
    use super::{parse_input, solve_banks};

    static TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn parse_test_input() {
        let banks = parse_input(TEST_INPUT);
        let expected = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        assert_eq!(banks, expected);
    }

    #[test]
    pub fn part_one_test_input() {
        let banks = parse_input(TEST_INPUT);
        let result = solve_banks(2, &banks);
        assert_eq!(result, 357);
    }

    #[test]
    pub fn part_one_known_answer() {
        let banks = parse_input(crate::days::get_input(3).unwrap());
        let result = solve_banks(2, &banks);
        assert_eq!(result, 17766);
    }

    #[test]
    pub fn part_two_test_input() {
        let banks = parse_input(TEST_INPUT);
        let result = solve_banks(12, &banks);
        assert_eq!(result, 3121910778619);
    }

    #[test]
    pub fn part_two_known_answer() {
        let banks = parse_input(crate::days::get_input(3).unwrap());
        let result = solve_banks(12, &banks);
        assert_eq!(result, 176582889354075);
    }
}

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let banks = parse_input(input);
    let p1 = solve_part_one(&banks);
    Answer::first(3, p1).report()
}

fn parse_input(s: &str) -> Vec<Vec<u8>> {
    s.trim()
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn solve_part_one(banks: &[Vec<u8>]) -> u32 {
    // Banks are all the same length, so precompute the last index.
    let last_idx = banks[0].len() - 1;
    let mut total_joltage = 0;
    for bank in banks {
        for needle in (0..=9).rev() {
            if let Some(idx) = bank.iter().position(|n| *n == needle)
                && idx != last_idx
            {
                let first = needle as u32;
                let &second = bank[idx + 1..].iter().max().unwrap();
                total_joltage += first * 10 + second as u32;
                break;
            }
        }
    }
    total_joltage
}

#[cfg(test)]
mod test {
    static TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn parse_test_input() {
        let banks = super::parse_input(TEST_INPUT);
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
        let banks = super::parse_input(TEST_INPUT);
        let result = super::solve_part_one(&banks);
        assert_eq!(result, 357)
    }

    #[test]
    pub fn part_one_known_answer() {
        let banks = super::parse_input(crate::days::get_input(3).unwrap());
        let result = super::solve_part_one(&banks);
        assert_eq!(result, 17766)
    }
}

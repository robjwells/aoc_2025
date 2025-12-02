use std::ops::RangeInclusive;

use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{newline, u64 as parse_u64},
    combinator::opt,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};

use crate::NumUtil;
use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let ranges = parse_input(input)?;
    let p1 = sum_invalid(&ranges, is_invalid_part_one);
    let p2 = sum_invalid(&ranges, is_invalid_part_two);
    Answer::first(2, p1).second(p2).report()
}

fn parse_input(s: &str) -> anyhow::Result<Vec<RangeInclusive<u64>>> {
    let mut parser = terminated(
        separated_list1(
            tag(","),
            separated_pair(
                parse_u64::<&str, nom::error::Error<&str>>,
                tag("-"),
                parse_u64,
            ),
        ),
        opt(newline),
    );
    let (leftover, pairs) = parser.parse(s).map_err(|e| e.to_owned())?;
    if !leftover.is_empty() {
        anyhow::bail!("Did not parse complete input, leftover: {leftover:?}");
    }
    let ranges = pairs.into_iter().map(|(start, end)| start..=end).collect();
    Ok(ranges)
}

/// Check if number is two repeated sequences of digits.
///
/// The multiplication check works by looking for a number of a particular length,
/// so eg `10` means a 2-digit number, `100` a three digit number, etc, and the final
/// `1` means that number repeated.
fn is_invalid_part_one(number: u64) -> bool {
    match number.n_digits() {
        2 => number.is_multiple_of(11),
        4 => number.is_multiple_of(101),
        6 => number.is_multiple_of(1001),
        8 => number.is_multiple_of(10001),
        10 => number.is_multiple_of(100001),
        _ => false,
    }
}

/// Check if number is composed entirely of repeated sequences of digits.
///
/// The multiplication check works by looking for a number of a particular length,
/// so eg `10` means a 2-digit number, `100` a three digit number, etc, and the final
/// `1` means that number repeated.
///
/// As opposed to part one, this can repeat several times (not just twice), so eg
/// 10101 means a 2 digit number repeated three times.
fn is_invalid_part_two(number: u64) -> bool {
    match number.n_digits() {
        2 => number.is_multiple_of(11),
        3 => number.is_multiple_of(111),
        4 => number.is_multiple_of(101),
        5 => number.is_multiple_of(11111),
        6 => number.is_multiple_of(1001) || number.is_multiple_of(10101),
        7 => number.is_multiple_of(1111111),
        8 => number.is_multiple_of(10001) || number.is_multiple_of(1010101),
        9 => number.is_multiple_of(111111111) || number.is_multiple_of(1001001),
        10 => number.is_multiple_of(100001) | number.is_multiple_of(101010101),
        _ => false,
    }
}

pub fn sum_invalid(ranges: &[RangeInclusive<u64>], check_function: fn(u64) -> bool) -> u64 {
    ranges
        .iter()
        .flat_map(|r| r.clone())
        .filter(|n| check_function(*n))
        .sum()
}

#[cfg(test)]
mod test {
    use super::{is_invalid_part_one, is_invalid_part_two, parse_input, sum_invalid};

    static TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn parse_test_input() -> anyhow::Result<()> {
        let res = parse_input(TEST_INPUT)?;
        let expected = vec![
            (11..=22),
            (95..=115),
            (998..=1012),
            (1188511880..=1188511890),
            (222220..=222224),
            (1698522..=1698528),
            (446443..=446449),
            (38593856..=38593862),
            (565653..=565659),
            (824824821..=824824827),
            (2121212118..=2121212124),
        ];
        assert_eq!(res, expected);
        Ok(())
    }

    #[test]
    pub fn part_one_test_input() -> anyhow::Result<()> {
        let ranges = parse_input(TEST_INPUT)?;
        let result = sum_invalid(&ranges, is_invalid_part_one);
        assert_eq!(result, 1227775554);
        Ok(())
    }

    #[test]
    pub fn part_one_known_answer() -> anyhow::Result<()> {
        let ranges = parse_input(crate::days::get_input(2).unwrap())?;
        let result = sum_invalid(&ranges, is_invalid_part_one);
        assert_eq!(result, 18595663903);
        Ok(())
    }

    #[test]
    pub fn part_two_test_input() -> anyhow::Result<()> {
        let ranges = parse_input(TEST_INPUT)?;
        let result = sum_invalid(&ranges, is_invalid_part_two);
        assert_eq!(result, 4174379265);
        Ok(())
    }

    #[test]
    pub fn part_two_known_answer() -> anyhow::Result<()> {
        let ranges = parse_input(crate::days::get_input(2).unwrap())?;
        let result = sum_invalid(&ranges, is_invalid_part_two);
        assert_eq!(result, 19058204438);
        Ok(())
    }
}

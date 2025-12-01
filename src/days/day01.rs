use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{i16 as parse_i16, newline},
    combinator::opt,
    multi::separated_list1,
    sequence::terminated,
};

use crate::util::Answer;

// static INPUT: &str = include_str!("../input/2025-01.txt");

pub fn solve(input: &str) -> anyhow::Result<String> {
    let rotations = parse_input(input).expect("Failed to parse real input.");
    let (p1, p2) = times_at_zero(50, &rotations);
    Answer::first(1, p1).second(p2).report()
}

fn parse_line(line: &str) -> IResult<&str, i16> {
    let mut parser = (alt((tag("L"), tag("R"))), parse_i16);
    let (leftover, (direction, value)) = parser.parse(line)?;
    let rotation = if direction == "L" { -value } else { value };
    Ok((leftover, rotation))
}

fn parse_input(s: &str) -> anyhow::Result<Vec<i16>> {
    let mut full = terminated(separated_list1(newline, parse_line), opt(newline));
    let (leftover, rotations) = full.parse(s).map_err(|e| e.to_owned())?;
    if !leftover.is_empty() {
        anyhow::bail!("Did not parse full input, leftover: {leftover:?}");
    }
    Ok(rotations)
}

fn times_at_zero(start: i16, rotations: &[i16]) -> (i16, i16) {
    let mut current = start;
    let mut turning_zeroes = 0;
    let mut only_end_zeroes = 0;
    for rotation in rotations {
        turning_zeroes += rotation.abs() / 100;
        let sub_100_delta = rotation % 100;

        let next = current + sub_100_delta;

        // Part two, count turning past zero.
        // Checking current != 0 avoids double-counting when a rotation *starts* at zero.
        if next <= 0 && current != 0 || next > 99 {
            turning_zeroes += 1;
        }

        // Part one, count only ending at zero.
        if next % 100 == 0 {
            only_end_zeroes += 1;
        }

        current = match next {
            ..0 => next + 100,
            0..=99 => next,
            100.. => next - 100,
        };
    }
    (only_end_zeroes, turning_zeroes)
}

#[cfg(test)]
mod test {
    use super::{parse_input, times_at_zero};

    static TEST_INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    static ENTIRELY_IN_RANGE: &str = "\
L50
R10
L10";

    #[test]
    fn parse_test_input() {
        let rotations = parse_input(TEST_INPUT).expect("Test input failed to parse");
        let expected = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
        assert_eq!(rotations, expected);
    }

    #[test]
    pub fn test_times_at_zero_part_one() {
        let rotations = parse_input(TEST_INPUT).expect("Test input failed to parse");
        let (p1, _) = times_at_zero(50, &rotations);
        assert_eq!(p1, 3);
    }

    #[test]
    pub fn test_times_at_zero_part_two() {
        let rotations = parse_input(TEST_INPUT).expect("Test input failed to parse");
        let (_, p2) = times_at_zero(50, &rotations);
        assert_eq!(p2, 6);
    }

    #[test]
    pub fn test_times_at_zero_part_two_in_range() {
        let rotations = parse_input(ENTIRELY_IN_RANGE).expect("In-range input failed to parse");
        let (_, p2) = times_at_zero(50, &rotations);
        assert_eq!(p2, 2);
    }

    #[test]
    pub fn test_times_at_zero_rotation_from_zero() {
        let rotations = parse_input("L50\nL10").expect("Rotation-from-zero input failed to parse");
        let (_, p2) = times_at_zero(50, &rotations);
        assert_eq!(p2, 1);
    }

    #[test]
    pub fn test_known_answers() {
        let input = include_str!("../../input/2025-01.txt");
        let rotations = parse_input(input).expect("Real input failed to parse");
        let result = times_at_zero(50, &rotations);
        assert_eq!(result, (999, 6099));
    }
}

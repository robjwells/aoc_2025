use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32 as parse_i32, newline},
    combinator::opt,
    multi::separated_list1,
    sequence::terminated,
};

static INPUT: &str = include_str!("../input/2025-01.txt");

fn main() {
    let rotations = parse_input(INPUT).expect("Failed to parse real input.");
    let p1 = times_at_zero(50, &rotations);
    println!("Part one: {p1}");
}

#[derive(Debug, Eq, PartialEq)]
enum Rotation {
    Left(i32),
    Right(i32),
}

fn parse_line(line: &str) -> IResult<&str, Rotation> {
    let mut parser = (alt((tag("L"), tag("R"))), parse_i32);
    let (leftover, (direction, value)) = parser.parse(line)?;
    let rotation = if direction == "L" {
        Rotation::Left(value)
    } else {
        Rotation::Right(value)
    };
    Ok((leftover, rotation))
}

fn parse_input(s: &str) -> anyhow::Result<Vec<Rotation>> {
    let mut full = terminated(separated_list1(newline, parse_line), opt(newline));
    let (leftover, rotations) = full.parse(s).map_err(|e| e.to_owned())?;
    if !leftover.is_empty() {
        anyhow::bail!("Did not parse full input, leftover: {leftover:?}");
    }
    Ok(rotations)
}

fn times_at_zero(start: i32, rotations: &[Rotation]) -> i32 {
    let mut current = start;
    let mut times = 0;
    for rotation in rotations {
        match rotation {
            Rotation::Left(n) => current -= *n,
            Rotation::Right(n) => current += *n,
        }
        if current % 100 == 0 {
            times += 1;
            current = 0;
        }
    }
    times
}

#[cfg(test)]
mod test {
    use crate::{Rotation, times_at_zero};

    use super::parse_input;

    static PART_ONE_TEST_INPUT: &str = "\
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

    #[test]
    fn parse_test_input() {
        let rotations = parse_input(PART_ONE_TEST_INPUT).expect("Test input failed to parse");
        let expected = vec![
            Rotation::Left(68),
            Rotation::Left(30),
            Rotation::Right(48),
            Rotation::Left(5),
            Rotation::Right(60),
            Rotation::Left(55),
            Rotation::Left(1),
            Rotation::Left(99),
            Rotation::Right(14),
            Rotation::Left(82),
        ];
        assert_eq!(rotations, expected);
    }

    #[test]
    pub fn test_times_at_zero() {
        let rotations = parse_input(PART_ONE_TEST_INPUT).expect("Test input failed to parse");
        let result = times_at_zero(50, &rotations);
        assert_eq!(result, 3);
    }

    #[test]
    pub fn test_part_one_known_answer() {
        let rotations = parse_input(super::INPUT).expect("Real input failed to parse");
        let result = times_at_zero(50, &rotations);
        assert_eq!(result, 999);
    }
}

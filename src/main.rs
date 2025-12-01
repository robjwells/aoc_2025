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
    let p2 = times_at_zero_part_two(50, &rotations);
    println!("Part one: {p1}");
    println!("Part two: {p2}");
}

#[derive(Debug, Eq, PartialEq)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl Rotation {
    fn delta(&self) -> i32 {
        match self {
            Rotation::Left(n) => -*n,
            Rotation::Right(n) => *n,
        }
    }

    fn value(&self) -> i32 {
        match self {
            Rotation::Left(n) => *n,
            Rotation::Right(n) => *n,
        }
    }
}

impl core::fmt::Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (c, n) = match self {
            Rotation::Left(n) => ('L', *n),
            Rotation::Right(n) => ('R', *n),
        };
        write!(f, "{c}{n:2}")
    }
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

fn times_at_zero_part_two(start: i32, rotations: &[Rotation]) -> i32 {
    let mut current = start;
    let mut times = 0;
    for rotation in rotations {
        let d = rotation.delta().signum();
        // Dumb but works.
        for _ in 0..rotation.value() {
            current += d;
            current %= 100;
            if current == 0 {
                times += 1;
            } else if current == -1 {
                current = 99;
            }
        }
    }
    times
}

#[cfg(test)]
mod test {
    use super::{Rotation, parse_input, times_at_zero, times_at_zero_part_two};

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

    static ENTIRELY_IN_RANGE: &str = "\
L50
R10
L10";

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

    #[test]
    pub fn test_times_at_zero_part_two() {
        let rotations = parse_input(PART_ONE_TEST_INPUT).expect("Test input failed to parse");
        let result = times_at_zero_part_two(50, &rotations);
        assert_eq!(result, 6);
    }

    #[test]
    pub fn test_times_at_zero_part_two_in_range() {
        let rotations = parse_input(ENTIRELY_IN_RANGE).expect("In-range input failed to parse");
        let result = times_at_zero_part_two(50, &rotations);
        assert_eq!(result, 2);
    }

    #[test]
    pub fn test_part_two_known_answer() {
        let rotations = parse_input(super::INPUT).expect("Real input failed to parse");
        let result = times_at_zero_part_two(50, &rotations);
        assert_eq!(result, 6099);
    }
}

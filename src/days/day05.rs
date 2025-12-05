#![allow(dead_code, unused_mut, unused)]
use std::{ops::RangeInclusive, str::FromStr};

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{newline, u64},
    combinator::opt,
    multi::separated_list1,
    sequence::separated_pair,
};

pub fn solve(input: &str) -> anyhow::Result<String> {
    todo!()
}

struct Database {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    available_ingredients: Vec<u64>,
}

impl FromStr for Database {
    type Err = nom::Err<nom::error::Error<String>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        _parse_input(s).map(|(_, db)| db).map_err(|e| e.to_owned())
    }
}

fn _parse_input(input: &str) -> IResult<&str, Database> {
    fn range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
        let (remaining, res) = separated_pair(u64, tag("-"), u64).parse(input)?;
        Ok((remaining, res.0..=res.1))
    }
    fn range_list(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
        separated_list1(newline, range).parse(input)
    }
    fn double_newline(input: &str) -> IResult<&str, &str> {
        tag("\n\n").parse(input)
    }
    fn ingredients_list(input: &str) -> IResult<&str, Vec<u64>> {
        separated_list1(newline, u64).parse(input)
    }
    fn trailing_newline(input: &str) -> IResult<&str, Option<char>> {
        opt(newline).parse(input)
    }

    let (remaining, ranges) = range_list(input)?;
    let (remaining, _) = double_newline(remaining)?;
    let (remaining, ingredients) = ingredients_list(remaining)?;
    let (remaining, _) = trailing_newline(remaining)?;
    assert!(remaining.is_empty(), "Leftover input: {remaining:?}");

    Ok((
        "",
        Database {
            fresh_ranges: ranges,
            available_ingredients: ingredients,
        },
    ))
}

#[cfg(test)]
mod test {

    use super::Database;

    static TEST_INPUT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn parse_test_input() -> anyhow::Result<()> {
        let database: Database = TEST_INPUT.parse()?;
        let expected_ranges = vec![3..=5, 10..=14, 16..=20, 12..=18];
        let expected_available = vec![1, 5, 8, 11, 17, 32];
        assert_eq!(database.fresh_ranges, expected_ranges);
        assert_eq!(database.available_ingredients, expected_available);
        Ok(())
    }

    #[test]
    fn parse_real_input() -> anyhow::Result<()> {
        let database: Database = crate::days::get_input(5).unwrap().parse()?;
        assert_eq!(database.fresh_ranges.len(), 190);
        assert_eq!(database.available_ingredients.len(), 1000);
        Ok(())
    }
}

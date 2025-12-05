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

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let database: Database = input.parse()?;
    let p1 = database.count_available_fresh();
    Answer::first(5, p1).report()
}

struct Database {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    available_ingredients: Vec<u64>,
}

impl Database {
    fn is_fresh(&self, ingredient: u64) -> bool {
        self.fresh_ranges.iter().any(|r| r.contains(&ingredient))
    }

    fn count_available_fresh(&self) -> usize {
        self.available_ingredients
            .iter()
            .filter(|i| self.is_fresh(**i))
            .count()
    }
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

    let (remaining, mut ranges) = range_list(input)?;
    let (remaining, _) = double_newline(remaining)?;
    let (remaining, mut ingredients) = ingredients_list(remaining)?;
    let (remaining, _) = trailing_newline(remaining)?;
    assert!(remaining.is_empty(), "Leftover input: {remaining:?}");

    // Ensure ranges and ingredients are sorted.
    ranges.sort_unstable_by_key(|r| (*r.start(), *r.end()));
    ingredients.sort_unstable();

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
        // Ranges and available ingredients are both sorted after parsing.
        let expected_ranges = vec![3..=5, 10..=14, 12..=18, 16..=20];
        let expected_available = vec![1, 5, 8, 11, 17, 32];
        assert_eq!(database.fresh_ranges, expected_ranges);
        assert_eq!(database.available_ingredients, expected_available);
        Ok(())
    }

    #[test]
    fn part_one_test_freshness() -> anyhow::Result<()> {
        let database: Database = TEST_INPUT.parse()?;
        assert!(!database.is_fresh(32));
        assert!(database.is_fresh(5));
        Ok(())
    }

    #[test]
    fn part_one_count_fresh() -> anyhow::Result<()> {
        let database: Database = TEST_INPUT.parse()?;
        let n_fresh = database.count_available_fresh();
        assert_eq!(3, n_fresh);
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

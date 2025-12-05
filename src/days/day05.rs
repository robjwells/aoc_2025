use std::{cmp::Ordering, collections::VecDeque, ops::RangeInclusive, str::FromStr};

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
    let p2 = database.count_all_fresh();
    Answer::first(5, p1).second(p2).report()
}

struct Database {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    available_ingredients: Vec<u64>,
}

impl Database {
    fn is_fresh(&self, ingredient: &u64) -> bool {
        // Use binary search to try to find a range that contains ingredient.
        // Not the logic is inverted: we exclude ranges that couldn't contain
        // the ingredient.
        self.fresh_ranges
            .binary_search_by(|probe| {
                if probe.start() > ingredient {
                    Ordering::Greater
                } else if probe.end() < ingredient {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .is_ok()
    }

    fn count_available_fresh(&self) -> usize {
        self.available_ingredients
            .iter()
            .filter(|i| self.is_fresh(i))
            .count()
    }

    fn count_all_fresh(&self) -> u64 {
        self.fresh_ranges
            .iter()
            .map(|r| *r.end() - *r.start() + 1)
            .sum()
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
    let (remaining, ingredients) = ingredients_list(remaining)?;
    let (remaining, _) = trailing_newline(remaining)?;
    assert!(remaining.is_empty(), "Leftover input: {remaining:?}");

    // Pre-emptively merge overlapping ranges.
    ranges = merge_ranges(ranges);

    Ok((
        "",
        Database {
            fresh_ranges: ranges,
            available_ingredients: ingredients,
        },
    ))
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    ranges.sort_unstable_by_key(|r| *r.start());
    let mut ranges: VecDeque<_> = ranges.into();
    let mut merged = Vec::new();

    while let Some(mut working) = ranges.pop_front() {
        while let Some(next) = ranges.pop_front() {
            if working.contains(next.start()) {
                let (start, end) = working.into_inner();
                working = start..=(end.max(*next.end()));
            } else {
                // Put the disjoint range back at the front of the queue.
                ranges.push_front(next);
                break;
            }
        }
        merged.push(working);
    }

    merged
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

    fn real_input() -> &'static str {
        crate::days::get_input(5).unwrap()
    }

    #[test]
    fn parse_test_input() -> anyhow::Result<()> {
        let database: Database = TEST_INPUT.parse()?;
        // Ranges are merged after parsing.
        let expected_ranges = vec![3..=5, 10..=20];
        let expected_available = vec![1, 5, 8, 11, 17, 32];
        assert_eq!(database.fresh_ranges, expected_ranges);
        assert_eq!(database.available_ingredients, expected_available);
        Ok(())
    }

    #[test]
    fn part_one_test_freshness() -> anyhow::Result<()> {
        let database: Database = TEST_INPUT.parse()?;
        assert!(!database.is_fresh(&32));
        assert!(database.is_fresh(&5));
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
    pub fn part_one_known_answer() -> anyhow::Result<()> {
        let database: Database = real_input().parse()?;
        let n_fresh = database.count_available_fresh();
        assert_eq!(868, n_fresh);
        Ok(())
    }

    #[test]
    pub fn part_two_count_all_fresh() -> anyhow::Result<()> {
        let database: Database = TEST_INPUT.parse()?;
        let n_all_fresh = database.count_all_fresh();
        assert_eq!(n_all_fresh, 14);
        Ok(())
    }

    #[test]
    pub fn part_two_known_answer() -> anyhow::Result<()> {
        let database: Database = real_input().parse()?;
        let n_fresh = database.count_all_fresh();
        assert_eq!(354143734113772, n_fresh);
        Ok(())
    }
}

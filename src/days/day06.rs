#![allow(unused, dead_code, unused_mut)]

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, space0, space1, u64},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
pub fn solve(input: &str) -> anyhow::Result<String> {
    todo!()
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Add,
    Multiply,
}

#[derive(Debug, Eq, PartialEq)]
struct Column {
    numbers: Vec<u64>,
    operator: Op,
}

impl From<(Vec<u64>, Op)> for Column {
    fn from((numbers, operator): (Vec<u64>, Op)) -> Self {
        Self { numbers, operator }
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Column>> {
    let (number_rows, operators) = _parse_rows(input)
        .map(|(_, res)| res)
        .map_err(|e| e.to_owned())?;
    // Transpose the number vecs
    let n_columns = number_rows[0].len();
    let mut iterators: Vec<_> = number_rows.into_iter().map(|row| row.into_iter()).collect();
    let number_columns: Vec<Vec<u64>> = (0..n_columns)
        .map(|_| {
            // For each column, pop a number from each row (ie, take a column at a time).
            iterators
                .iter_mut()
                .map(|row| row.next().unwrap())
                .collect()
        })
        .collect();
    // Pair with operator in Column struct
    let columns = number_columns
        .into_iter()
        .zip(operators)
        .map(Column::from)
        .collect();
    Ok(columns)
}

fn _parse_rows(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<Op>)> {
    fn numbers(input: &str) -> IResult<&str, Vec<u64>> {
        let (remaining, _) = space0(input)?;
        let (remaining, numbers) = separated_list1(space1, u64).parse(remaining)?;
        let (remaining, _) = space0(remaining)?;
        Ok((remaining, numbers))
    }
    fn number_rows(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
        separated_list1(newline, numbers).parse(input)
    }
    fn operator(input: &str) -> IResult<&str, Op> {
        let (leftover, op_char) = alt((tag("+"), tag("*"))).parse(input)?;
        let op = match op_char {
            "+" => Op::Add,
            "*" => Op::Multiply,
            _ => unreachable!(),
        };
        Ok((leftover, op))
    }
    fn operator_row(input: &str) -> IResult<&str, Vec<Op>> {
        separated_list1(space1, operator).parse(input)
    }
    let (remaining, rows) = separated_pair(number_rows, newline, operator_row).parse(input)?;
    assert!(remaining.trim_start().is_empty());
    Ok((remaining, rows))
}

#[cfg(test)]
mod test {
    use super::{Column, Op, parse_input};

    static TEST_INPUT: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

    #[test]
    fn parse_test_input() -> anyhow::Result<()> {
        let expected = vec![
            Column {
                numbers: vec![123, 45, 6],
                operator: Op::Multiply,
            },
            Column {
                numbers: vec![328, 64, 98],
                operator: Op::Add,
            },
            Column {
                numbers: vec![51, 387, 215],
                operator: Op::Multiply,
            },
            Column {
                numbers: vec![64, 23, 314],
                operator: Op::Add,
            },
        ];
        let result = parse_input(TEST_INPUT)?;
        assert_eq!(result, expected);
        Ok(())
    }
}

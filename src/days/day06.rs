use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let p1 = sum_calculated_groups(&part_one::parse_input(input)?);
    let p2 = sum_calculated_groups(&part_two::parse_input(input)?);
    Answer::first(6, p1).second(p2).report()
}

fn sum_calculated_groups(groups: &[Group]) -> u64 {
    groups.iter().map(Group::apply).sum()
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Add,
    Multiply,
}

impl Op {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Multiply => a * b,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Group {
    numbers: Vec<u64>,
    operator: Op,
}

impl From<(Vec<u64>, Op)> for Group {
    fn from((numbers, operator): (Vec<u64>, Op)) -> Self {
        Self { numbers, operator }
    }
}

impl Group {
    fn apply(&self) -> u64 {
        self.numbers
            .iter()
            .copied()
            .reduce(|a, b| self.operator.apply(a, b))
            .unwrap()
    }
}

fn transpose<T>(rows: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let Some(n_columns) = rows.first().map(Vec::len) else {
        // Rows is empty
        return rows;
    };
    let mut iterators: Vec<_> = rows.into_iter().map(|row| row.into_iter()).collect();
    (0..n_columns)
        .map(|_| {
            // For each column, pop a number from each row (ie, take a column at a time).
            iterators
                .iter_mut()
                .map(|row| row.next().unwrap())
                .collect()
        })
        .collect()
}

mod part_one {
    use nom::{
        IResult, Parser,
        branch::alt,
        bytes::complete::tag,
        character::complete::{newline, space0, space1, u64},
        multi::separated_list1,
        sequence::separated_pair,
    };

    use crate::days::day06::transpose;

    use super::{Group, Op};

    pub(super) fn parse_input(input: &str) -> anyhow::Result<Vec<Group>> {
        let (number_rows, operators) = _parse_rows(input)
            .map(|(_, res)| res)
            .map_err(|e| e.to_owned())?;
        // Transpose the number vecs
        let number_columns = transpose(number_rows);
        // Pair with operator in Column struct
        let columns = number_columns
            .into_iter()
            .zip(operators)
            .map(Group::from)
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
}

mod part_two {
    use std::collections::VecDeque;

    use crate::days::day06::transpose;

    use super::{Group, Op};

    pub(super) fn parse_input(input: &str) -> anyhow::Result<Vec<Group>> {
        let (number_lines, op_line) = input.trim_end().rsplit_once("\n").unwrap();
        let number_rows_as_chars: Vec<Vec<char>> =
            number_lines.lines().map(|e| e.chars().collect()).collect();
        let mut columns = VecDeque::from(transpose(number_rows_as_chars));
        let mut number_groups = Vec::new();
        while !columns.is_empty() {
            let mut current_group = Vec::with_capacity(4);
            while let Some(col) = columns.pop_front() {
                if col.iter().all(|&c| c == ' ') {
                    break;
                }
                current_group.push(parse_column(col));
            }
            number_groups.push(current_group);
        }

        let groups = number_groups
            .into_iter()
            .zip(parse_operator_line(op_line))
            .map(Group::from)
            .collect();
        Ok(groups)
    }

    fn parse_column(column: Vec<char>) -> u64 {
        let mut total = 0;
        for c in column {
            if let Some(d) = c.to_digit(10) {
                total = (total * 10) + (d as u64);
            }
        }
        total
    }

    fn parse_operator_line(line: &str) -> impl IntoIterator<Item = Op> {
        line.chars().filter(|&c| c != ' ').map(|c| match c {
            '+' => Op::Add,
            '*' => Op::Multiply,
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Group, Op, part_one, part_two, sum_calculated_groups};

    static TEST_INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314 \n*   +   *   +  \n";

    #[test]
    fn parse_test_input_part_one() -> anyhow::Result<()> {
        let expected = vec![
            Group {
                numbers: vec![123, 45, 6],
                operator: Op::Multiply,
            },
            Group {
                numbers: vec![328, 64, 98],
                operator: Op::Add,
            },
            Group {
                numbers: vec![51, 387, 215],
                operator: Op::Multiply,
            },
            Group {
                numbers: vec![64, 23, 314],
                operator: Op::Add,
            },
        ];
        let result = part_one::parse_input(TEST_INPUT)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    pub fn part_one_test_input() -> anyhow::Result<()> {
        let groups = part_one::parse_input(TEST_INPUT)?;
        let result = sum_calculated_groups(&groups);
        let expected = 4277556;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    pub fn part_one_known_answer() -> anyhow::Result<()> {
        let groups = part_one::parse_input(crate::days::get_input(6).unwrap())?;
        let result = sum_calculated_groups(&groups);
        let expected = 6503327062445;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn parse_test_input_part_two() -> anyhow::Result<()> {
        let expected = vec![
            Group {
                numbers: vec![1, 24, 356],
                operator: Op::Multiply,
            },
            Group {
                numbers: vec![369, 248, 8],
                operator: Op::Add,
            },
            Group {
                numbers: vec![32, 581, 175],
                operator: Op::Multiply,
            },
            Group {
                numbers: vec![623, 431, 4],
                operator: Op::Add,
            },
        ];
        let result = part_two::parse_input(TEST_INPUT)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    pub fn part_two_test_input() -> anyhow::Result<()> {
        let groups = part_two::parse_input(TEST_INPUT)?;
        let result = sum_calculated_groups(&groups);
        let expected = 3263827;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    pub fn part_two_known_answer() -> anyhow::Result<()> {
        let groups = part_two::parse_input(crate::days::get_input(6).unwrap())?;
        let result = sum_calculated_groups(&groups);
        let expected = 9640641878593;
        assert_eq!(result, expected);
        Ok(())
    }
}

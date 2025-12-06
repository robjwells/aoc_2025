use std::fmt::Display;

pub struct Answer {
    day: usize,
    first: String,
    second: String,
}
pub struct PartialAnswer {
    day: usize,
    first: String,
}

impl Answer {
    pub fn first<T: Display>(day: usize, answer: T) -> PartialAnswer {
        PartialAnswer {
            day,
            first: answer.to_string(),
        }
    }

    pub fn report(self) -> anyhow::Result<String> {
        Ok(self.to_string())
    }
}

impl PartialAnswer {
    pub fn second<T: Display>(self, answer: T) -> Answer {
        Answer {
            day: self.day,
            first: self.first,
            second: answer.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn report(self) -> anyhow::Result<String> {
        Ok(self.to_string())
    }
}

impl Display for PartialAnswer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Day {}", self.day)?;
        writeln!(f, "==========================")?;
        writeln!(f, "Part one: {:>16}", self.first)
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Day {}", self.day)?;
        writeln!(f, "==========================")?;
        writeln!(f, "Part one: {:>16}", self.first)?;
        writeln!(f, "Part two: {:>16}", self.second)
    }
}

#[rstest::fixture]
#[once]
fn tracing_fixture() -> () {
    tracing_subscriber::fmt::init();
}

pub fn transpose<T>(rows: Vec<Vec<T>>) -> Vec<Vec<T>> {
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

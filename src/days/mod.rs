pub mod day01;

static INPUT: &[&str] = &[include_str!("../../input/2025-01.txt")];

static SOLVERS: &[fn(&str) -> anyhow::Result<String>] = &[day01::solve];

pub fn get_input(day: usize) -> Option<&'static str> {
    INPUT.get(day - 1).copied()
}

pub fn get_solver(day: usize) -> Option<&'static fn(&str) -> anyhow::Result<String>> {
    SOLVERS.get(day - 1)
}

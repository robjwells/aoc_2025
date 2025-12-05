pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

static INPUT: &[&str] = &[
    include_str!("../../input/2025-01.txt"),
    include_str!("../../input/2025-02.txt"),
    include_str!("../../input/2025-03.txt"),
    include_str!("../../input/2025-04.txt"),
    include_str!("../../input/2025-05.txt"),
];

static SOLVERS: &[fn(&str) -> anyhow::Result<String>] =
    &[day01::solve, day02::solve, day03::solve, day04::solve, day05::solve];

pub fn get_input(day: usize) -> Option<&'static str> {
    INPUT.get(day - 1).copied()
}

pub fn get_solver(day: usize) -> Option<&'static fn(&str) -> anyhow::Result<String>> {
    SOLVERS.get(day - 1)
}

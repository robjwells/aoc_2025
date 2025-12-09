pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;

static INPUT: &[&str] = &[
    include_str!("../../input/2025-01.txt"),
    include_str!("../../input/2025-02.txt"),
    include_str!("../../input/2025-03.txt"),
    include_str!("../../input/2025-04.txt"),
    include_str!("../../input/2025-05.txt"),
    include_str!("../../input/2025-06.txt"),
    include_str!("../../input/2025-07.txt"),
    include_str!("../../input/2025-08.txt"),
];

static SOLVERS: &[fn(&str) -> anyhow::Result<String>] = &[
    day01::solve,
    day02::solve,
    day03::solve,
    day04::solve,
    day05::solve,
    day06::solve,
    day07::solve,
    day08::solve,
];

pub fn get_input(day: usize) -> Option<&'static str> {
    INPUT.get(day - 1).copied()
}

pub fn get_solver(day: usize) -> Option<&'static fn(&str) -> anyhow::Result<String>> {
    SOLVERS.get(day - 1)
}

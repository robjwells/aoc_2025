fn main() -> anyhow::Result<()> {
    // Enable `tracing` logging.
    tracing_subscriber::fmt::init();

    let arg = std::env::args().nth(1);
    let Some(Ok(day)) = arg.map(|s| s.parse::<usize>()) else {
        anyhow::bail!("You must give the day to run.");
    };
    let solution = aoc_2025::run(day)?;
    println!("{solution}");

    Ok(())
}

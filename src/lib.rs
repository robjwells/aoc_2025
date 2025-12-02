pub mod days;
mod util;

#[tracing::instrument]
pub fn run(day: usize) -> anyhow::Result<String> {
    assert_ne!(day, 0, "Day must be >= 1.");

    let Some(input) = days::get_input(day) else {
        anyhow::bail!("No input for day {day}.");
    };
    let Some(day_fn) = days::get_solver(day) else {
        anyhow::bail!("Day {day} is not implemented yet.");
    };
    day_fn(input)
}

/// Handy extension methods for numeric types.
trait NumUtil {
    fn n_digits(&self) -> u32;
}

macro_rules! impl_NumUtil {
    ($($t:ty)+) => {
        $(
            impl NumUtil for $t {
                fn n_digits(&self) -> u32 {
                    self.ilog10() + 1
                }
            }
        )+
    };
}

impl_NumUtil!(u8 u16 u32 u64);

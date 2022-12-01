
pub mod part1;
pub mod part2;

pub use part1::run as run_part1;
pub use part2::run as run_part2;

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_13() {
        run();
    }
}

                            
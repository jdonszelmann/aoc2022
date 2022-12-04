pub mod part1;
pub mod part2;

use itertools::{Chunk, Itertools};
pub use part1::run as run_part1;
pub use part2::run as run_part2;

pub fn parse(inp: &str) -> impl Iterator<Item = ((u64, u64), (u64, u64))> + '_ {
    inp.split('\n')
        .flat_map(|i| i.split(','))
        .flat_map(|i| i.split('-'))
        .map(|i| i.parse::<u64>().unwrap())
        .tuples()
        .tuples()
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_4() {
        run();
    }
}

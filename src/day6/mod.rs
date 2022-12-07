pub mod part1;
pub mod part2;

use array_windows::ArrayWindowsExt;
use itertools::Itertools;
pub use part1::run as run_part1;
pub use part2::run as run_part2;
use std::collections::HashSet;
use std::hash::Hash;

pub fn first_subsequence_length<T: Hash + Eq + Clone, const N: usize>(
    seq: impl Iterator<Item = T>,
) -> usize {
    seq.array_windows::<N>()
        .map::<HashSet<T>, _>(HashSet::from_iter)
        .find_position(|i| i.len() == N)
        .unwrap()
        .0
        + N
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_6() {
        run();
    }
}

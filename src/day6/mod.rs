pub mod part1;
pub mod part2;

pub use part1::run as run_part1;
pub use part2::run as run_part2;
use std::collections::HashSet;
use std::hash::Hash;

pub fn first_subsequence_length<T: Hash + Eq>(seq: Vec<T>, n: usize) -> usize {
    seq.windows(n)
        .map::<HashSet<&T>, _>(HashSet::from_iter)
        .enumerate()
        .find(|(_, i)| i.len() == n)
        .unwrap()
        .0
        + n
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

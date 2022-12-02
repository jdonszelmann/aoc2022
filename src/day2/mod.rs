pub mod part1;
pub mod part2;

pub use part1::run as run_part1;
pub use part2::run as run_part2;

pub fn parse(inp: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    inp.split('\n')
        .filter(|i| !i.trim().is_empty())
        .map(|i| i.split(' '))
        .map(|mut i| {
            (
                i.next().expect("one element").chars().next().unwrap(),
                i.next().expect("two elements").chars().next().unwrap(),
            )
        })
        .map(|(a, b)| (a as u64 - 'A' as u64, b as u64 - 'X' as u64))
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_2() {
        run();
    }
}

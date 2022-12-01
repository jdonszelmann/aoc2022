pub mod part1;
pub mod part2;

pub use part1::run as run_part1;
pub use part2::run as run_part2;

pub fn parse(inp: &str) -> impl Iterator<Item = impl Iterator<Item = &str>> {
    inp.split("\n\n")
        .map(|i| i.split('\n').filter(|i| i.trim() != ""))
}

pub fn elf_sums(inp: &str) -> Vec<i64> {
    parse(inp)
        .map(|i| i.map(|j| j.parse::<i64>().expect("integer")))
        .map(|i| i.sum())
        .collect()
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_1() {
        run();
    }
}

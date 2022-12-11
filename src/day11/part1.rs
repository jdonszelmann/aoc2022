use crate::day11::simulate;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 11 part 1");

    let contents = read_to_string("src/day11/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> u64 {
    simulate(inp, 20, true)
}

#[cfg(test)]
mod tests {
    use crate::day11::part1::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_11_part_1() {
        let contents = read_to_string("src/day11/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 117624)
    }

    #[test]
    pub fn test_day_11_part_1_test_input() {
        let contents = read_to_string("src/day11/data.test").expect("no input file found");
        assert_eq!(implementation(&contents), 10605)
    }
}

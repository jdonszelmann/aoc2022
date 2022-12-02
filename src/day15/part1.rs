use crate::day2::parse;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 15 part 1");

    let contents = read_to_string("src/day15/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::day15::part1::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_15_part_1() {
        let contents = read_to_string("src/day15/data.in").expect("no input file found");
    }

    #[test]
    pub fn test_day_15_part_1_test_input() {
        let testdata = "";
    }
}

use crate::day2::parse;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 18 part 1");

    let contents = read_to_string("src/day18/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::day18::part1::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_18_part_1() {
        let contents = read_to_string("src/day18/data.in").expect("no input file found");
    }

    #[test]
    pub fn test_day_18_part_1_test_input() {
        let testdata = "";
    }
}

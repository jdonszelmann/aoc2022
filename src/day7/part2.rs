use crate::day2::parse;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 7 part 2");

    let contents = read_to_string("src/day7/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::day7::part1::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_7_part_2() {
        let contents = read_to_string("src/day7/data.in").expect("no input file found");
    }

    #[test]
    pub fn test_day_7_part_2_test_input() {
        let testdata = "";
    }
}

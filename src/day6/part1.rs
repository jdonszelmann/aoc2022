use crate::day2::parse;
use crate::day6::first_subsequence_length;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 6 part 1");

    let contents = read_to_string("src/day6/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> usize {
    first_subsequence_length(inp.chars().collect(), 4)
}

#[cfg(test)]
mod tests {
    use crate::day6::part1::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_6_part_1() {
        let contents = read_to_string("src/day6/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 1582)
    }

    #[test]
    pub fn test_day_6_part_1_test_input() {
        assert_eq!(implementation("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(implementation("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(implementation("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(implementation("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(implementation("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}

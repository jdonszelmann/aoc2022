use crate::day13::parse_list;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 13 part 1");

    let contents = read_to_string("src/day13/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> usize {
    inp.lines()
        .filter(|i| !i.is_empty())
        .map(parse_list)
        .tuples()
        .positions(|(l, r)| l < r)
        .map(|i| i + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day13::parse_list;
    use crate::day13::part1::implementation;
    use std::cmp::Ordering;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_13_part_1() {
        let contents = read_to_string("src/day13/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 6272);
    }

    #[test]
    pub fn test_day_13_part_1_test_input() {
        let contents = read_to_string("src/day13/data.test").expect("no input file found");
        assert_eq!(implementation(&contents), 13);
    }
}

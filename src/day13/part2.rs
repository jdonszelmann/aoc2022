use crate::day13::parse_list;
use crate::day2::parse;
use itertools::Itertools;
use std::fs::read_to_string;
use std::ops::Index;

pub fn run() {
    println!("aoc 2022 day 13 part 2");

    let contents = read_to_string("src/day13/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> usize {
    let mut elements = inp
        .lines()
        .filter(|i| !i.is_empty())
        .chain(vec!["[[2]]", "[[6]]"])
        .map(parse_list)
        .collect_vec();
    elements.sort();

    let m1 = elements
        .iter()
        .position(|i| i == &parse_list("[[2]]"))
        .unwrap()
        + 1;
    let m2 = elements
        .iter()
        .position(|i| i == &parse_list("[[6]]"))
        .unwrap()
        + 1;

    m1 * m2
}

#[cfg(test)]
mod tests {
    use crate::day13::part2::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_13_part_2() {
        let contents = read_to_string("src/day13/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 22288);
    }

    #[test]
    pub fn test_day_13_part_2_test_input() {
        let contents = read_to_string("src/day13/data.test").expect("no input file found");
        assert_eq!(implementation(&contents), 140);
    }
}

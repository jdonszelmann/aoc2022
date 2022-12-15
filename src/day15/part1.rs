use crate::day15::RangeSet;
use crate::day15::{parse, range_at_y};
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 15 part 1");

    let contents = read_to_string("src/day15/data.in").expect("no input file found");
    println!("{}", implementation(&contents, 2000000));
}

pub fn implementation(inp: &str, critical_y: u64) -> usize {
    let mut ranges = RangeSet::new();

    for (sensor, beacon) in parse(inp) {
        if let Some(r) = range_at_y(sensor, beacon, critical_y, None) {
            ranges.add(r)
        }
    }

    ranges.len()
}

#[cfg(test)]
mod tests {
    use crate::day15::part1::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_15_part_1() {
        let contents = read_to_string("src/day15/data.in").expect("no input file found");
        assert_eq!(implementation(&contents, 2000000), 4876693);
    }

    #[test]
    pub fn test_day_15_part_1_test_input() {
        let contents = read_to_string("src/day15/data.test").expect("no input file found");
        assert_eq!(implementation(&contents, 10), 26);
    }
}

use crate::day15::parse;
use crate::day15::{range_at_y, RangeSet};
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 15 part 2");

    let contents = read_to_string("src/day15/data.in").expect("no input file found");
    println!("{}", implementation(&contents, 4000000));
}

pub fn implementation(inp: &str, max_xy: u64) -> u64 {
    let data = parse(inp).collect::<Vec<_>>();
    for y in 0..max_xy {
        let mut ranges = RangeSet::new();
        for &(sensor, beacon) in &data {
            if let Some(r) = range_at_y(sensor, beacon, y, Some(max_xy)) {
                ranges.add(r)
            }
        }
        if ranges.len() != max_xy as usize {
            let x = (0..max_xy as i64).find(|i| !ranges.contains(*i)).unwrap() as u64;
            return x * 4000000 + y;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::day15::part2::implementation;
    use std::fs::read_to_string;

    #[test]
    #[ignore]
    pub fn test_day_15_part_2() {
        let contents = read_to_string("src/day15/data.in").expect("no input file found");
        assert_eq!(implementation(&contents, 4000000), 11645454855041)
    }

    #[test]
    pub fn test_day_15_part_2_test_input() {
        let contents = read_to_string("src/day15/data.test").expect("no input file found");
        assert_eq!(implementation(&contents, 20), 56000011)
    }
}

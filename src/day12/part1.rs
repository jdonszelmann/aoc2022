use crate::day12::pathfind;
use crate::day2::parse;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 12 part 1");

    let contents = read_to_string("src/day12/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> usize {
    pathfind(inp, &['S']).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day12::part1::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_12_part_1() {
        let contents = read_to_string("src/day12/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 380)
    }

    #[test]
    pub fn test_day_12_part_1_test_input() {
        let contents = read_to_string("src/day12/data.test").expect("no input file found");
        assert_eq!(implementation(&contents), 31);
    }
}

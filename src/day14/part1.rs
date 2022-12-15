use crate::day14::simulate;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 14 part 1");

    let contents = read_to_string("src/day14/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> usize {
    simulate(inp, true)
}

#[cfg(test)]
mod tests {
    use crate::day14::part1::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_14_part_1() {
        let contents = read_to_string("src/day14/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 763);
    }

    #[test]
    pub fn test_day_14_part_1_test_input() {
        let contents = read_to_string("src/day14/data.test").expect("no input file found");
        assert_eq!(implementation(&contents), 24);
    }
}

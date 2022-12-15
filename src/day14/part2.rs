use crate::day14::simulate;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 14 part 2");

    let contents = read_to_string("src/day14/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> usize {
    simulate(inp, false)
}

#[cfg(test)]
mod tests {
    use crate::day14::part2::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_14_part_2() {
        let contents = read_to_string("src/day14/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 23921);
    }

    #[test]
    pub fn test_day_14_part_2_test_input() {
        let contents = read_to_string("src/day14/data.test").expect("no input file found");
        assert_eq!(implementation(&contents), 93);
    }
}

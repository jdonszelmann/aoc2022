use crate::day9::simulate_snek;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 9 part 1");

    let contents = read_to_string("src/day9/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> usize {
    simulate_snek::<1>(inp)
}

#[cfg(test)]
mod tests {
    use crate::day9::part1::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_9_part_1() {
        let contents = read_to_string("src/day9/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 5878);
    }
}

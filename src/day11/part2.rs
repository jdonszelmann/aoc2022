use crate::day11::simulate;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 11 part 2");

    let contents = read_to_string("src/day11/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> u64 {
    simulate(inp, 10_000, false)
}

#[cfg(test)]
mod tests {
    use crate::day11::part2::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_11_part_2() {
        let contents = read_to_string("src/day11/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 16792940265);
    }

    #[test]
    pub fn test_day_11_part_2_test_input() {
        let contents = read_to_string("src/day11/data.test").expect("no input file found");
        assert_eq!(implementation(&contents), 2713310158);
    }
}

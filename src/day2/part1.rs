use crate::day2::parse;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 2 part 1");

    let contents = read_to_string("src/day2/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> u64 {
    parse(inp)
        .map(|i| match i {
            (0, 0) => 4,
            (0, 1) => 8,
            (0, 2) => 3,
            (1, 0) => 1,
            (1, 1) => 5,
            (1, 2) => 9,
            (2, 0) => 7,
            (2, 1) => 2,
            (2, 2) => 6,
            _ => unreachable!(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day2::part1::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_2_part_1() {
        let contents = read_to_string("src/day2/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 17189);
    }

    #[test]
    pub fn test_day_2_part_1_test_input() {
        assert_eq!(
            implementation(
                "A Y
B X
C Z"
            ),
            15
        );
    }
}

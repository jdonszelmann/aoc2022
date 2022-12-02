use crate::day2::parse;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 2 part 2");

    let contents = read_to_string("src/day2/input_part1.txt").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> u64 {
    parse(inp)
        .map(|i| match i {
            (0, 0) => 3,
            (0, 1) => 4,
            (0, 2) => 8,
            (1, 0) => 1,
            (1, 1) => 5,
            (1, 2) => 9,
            (2, 0) => 2,
            (2, 1) => 6,
            (2, 2) => 7,
            _ => unreachable!(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day2::part2::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_2_part_2() {
        let contents = read_to_string("src/day2/input_part1.txt").expect("no input file found");
        assert_eq!(implementation(&contents), 13490);
    }

    #[test]
    pub fn test_day_2_part_2_test_input() {
        assert_eq!(
            implementation(
                "A Y
B X
C Z"
            ),
            12
        );
    }
}

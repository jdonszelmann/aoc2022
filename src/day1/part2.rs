use super::elf_sums;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 1 part 2");

    let input = read_to_string("src/day1/input_part1.txt").expect("no input file found");
    println!("{}", implementation(&input));
}

pub fn implementation(inp: &str) -> i64 {
    let mut sums = elf_sums(inp);
    sums.sort();

    sums.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use crate::day1::part2::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_1_part_2_real_input() {
        let input = read_to_string("src/day1/input_part1.txt").expect("no input file found");
        assert_eq!(implementation(&input), 208567);
    }

    #[test]
    pub fn test_day_1_part_2_test_input() {
        assert_eq!(
            implementation(
                "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
        "
            ),
            45000
        );
    }
}

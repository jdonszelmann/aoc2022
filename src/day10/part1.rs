use crate::day10::run_cpu;
use crate::day2::parse;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 10 part 1");

    let contents = read_to_string("src/day10/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> u64 {
    let mut total = 0;
    run_cpu(inp, |x, y, c, sprite_x, hit| {
        if (c as i64 - 20) % 40 == 0 {
            total += (c as i64 * sprite_x as i64)
        }
    });

    total as u64
}

#[cfg(test)]
mod tests {
    use crate::day10::part1::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_10_part_1() {
        let contents = read_to_string("src/day10/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 13680)
    }

    #[test]
    pub fn test_day_10_part_1_test_input_1() {
        let contents = read_to_string("src/day10/1.test").expect("no input file found");
        assert_eq!(implementation(&contents), 0)
    }

    #[test]
    pub fn test_day_10_part_1_test_input_2() {
        let contents = read_to_string("src/day10/2.test").expect("no input file found");
        assert_eq!(implementation(&contents), 13140)
    }
}

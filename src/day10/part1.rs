use std::fs::read;

pub fn run() {
    println!("aoc 2022 day 10 part 1");

    let _input = read("src/day10/input_part1.txt").expect("no input file found");
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_10_part_1() {
        run();
    }
}

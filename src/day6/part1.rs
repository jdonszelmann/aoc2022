
use std::fs::read;
                
pub fn run() {
    println!("aoc 2022 day 6 part 1");

    let _input = read("/home/jonathan/Documents/projects/aoc2022/src/day6/input_part1.txt").expect("no input file found");
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_6_part_1() {
        run();
    }
}
        
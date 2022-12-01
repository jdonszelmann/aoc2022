
use std::fs::read;

pub fn run() {
    println!("aoc 2022 day 1 part 2");

    let _input = read("/home/jonathan/Documents/projects/aoc2022/src/day1/input_part1.txt").expect("no input file found");
}


#[cfg(test)]
mod tests {
    use super::run;
    
    #[test]
    pub fn test_day_1_part_2() {
        run();
    }
}
                    
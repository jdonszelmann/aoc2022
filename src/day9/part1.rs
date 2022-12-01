use std::fs::read;

pub fn run() {
    println!("aoc 2022 day 9 part 1");

    let _input = read("src/day9/input_part1.txt").expect("no input file found");
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_9_part_1() {
        run();
    }
}

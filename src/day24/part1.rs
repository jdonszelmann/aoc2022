use std::fs::read;

pub fn run() {
    println!("aoc 2022 day 24 part 1");

    let _input = read("src/day24/input_part1.txt").expect("no input file found");
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_24_part_1() {
        run();
    }
}

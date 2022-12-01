use std::fs::read;

pub fn run() {
    println!("aoc 2022 day 18 part 2");

    let _input = read("src/day18/input_part1.txt").expect("no input file found");
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_18_part_2() {
        run();
    }
}

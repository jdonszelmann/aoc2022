use std::fs::read;

pub fn run() {
    println!("aoc 2022 day 14 part 1");

    let _input = read("/home/jonathan/Documents/projects/aoc2022/src/day14/input_part1.txt")
        .expect("no input file found");
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_14_part_1() {
        run();
    }
}

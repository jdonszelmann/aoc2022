use std::fs::read;

pub fn run() {
    println!("aoc 2022 day 8 part 2");

    let _input = read("/home/jonathan/Documents/projects/aoc2022/src/day8/input_part1.txt")
        .expect("no input file found");
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_8_part_2() {
        run();
    }
}

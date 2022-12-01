use std::fs::read;

pub fn run() {
    println!("aoc 2022 day 23 part 1");

    let _input = read("/home/jonathan/Documents/projects/aoc2022/src/day23/input_part1.txt")
        .expect("no input file found");
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_23_part_1() {
        run();
    }
}

use crate::day12::pathfind;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 12 part 2");

    let contents = read_to_string("src/day12/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> usize {
    pathfind(inp, &['a', 'S']).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day12::part2::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_12_part_2() {
        let contents = read_to_string("src/day12/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 375)
    }

    #[test]
    pub fn test_day_12_part_2_test_input() {
        let contents = read_to_string("src/day12/data.test").expect("no input file found");
        assert_eq!(implementation(&contents), 29);
    }
}

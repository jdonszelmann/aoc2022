use crate::day8::{iter_grid, parse, walk_to_edge};
use std::fs::read_to_string;
use std::iter;

pub fn run() {
    println!("aoc 2022 day 8 part 1");

    let contents = read_to_string("src/day8/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn visible(start: (isize, isize), data: &[Vec<u8>], width: usize, height: usize) -> bool {
    let get_height = |(x, y): (isize, isize)| data[x as usize][y as usize];
    let start_height = get_height(start);

    [(-1, 0), (1, 0), (0, 1), (0, -1)]
        .into_iter()
        .any(|direction| {
            walk_to_edge(start, direction, width, height).all(|c| get_height(c) < start_height)
        })
}

pub fn implementation(inp: &str) -> usize {
    let (width, height, data) = parse(inp);

    iter_grid(width, height)
        .filter(|&c| visible(c, &data, width, height))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day8::part1::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_8_part_1() {
        let contents = read_to_string("src/day8/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 1711)
    }

    #[test]
    pub fn test_day_8_part_1_test_input() {
        let testdata = "30373
25512
65332
33549
35390";
        assert_eq!(implementation(testdata), 21)
    }
}

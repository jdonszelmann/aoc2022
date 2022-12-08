use crate::day8::{iter_grid, parse, walk_to_edge};
use crate::take_while_inclusive::TakeWhileInclusiveExt;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 8 part 2");

    let contents = read_to_string("src/day8/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn score(start: (isize, isize), data: &[Vec<u8>], width: usize, height: usize) -> usize {
    let get_height = |(x, y): (isize, isize)| data[x as usize][y as usize];

    let start_height = get_height(start);

    [(-1, 0), (1, 0), (0, 1), (0, -1)]
        .into_iter()
        .map(|direction| {
            walk_to_edge(start, direction, width, height)
                .take_while_inclusive(|&c| get_height(c) < start_height)
                .count()
        })
        .product()
}

pub fn implementation(inp: &str) -> usize {
    let (width, height, data) = parse(inp);

    iter_grid(width, height)
        .map(|c| score(c, &data, width, height))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day8::part2::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_8_part_2() {
        let contents = read_to_string("src/day8/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 301392)
    }

    #[test]
    pub fn test_day_8_part_2_test_input() {
        let testdata = "30373
25512
65332
33549
35390";
        assert_eq!(implementation(testdata), 8);
    }
}

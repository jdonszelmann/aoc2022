pub mod part1;
pub mod part2;

use itertools::iproduct;
pub use part1::run as run_part1;
pub use part2::run as run_part2;
use std::iter;

pub fn iter_grid(width: usize, height: usize) -> impl Iterator<Item = (isize, isize)> {
    iproduct!(0..(width as isize), 0..(height as isize))
}

pub fn walk(
    start: (isize, isize),
    direction: (isize, isize),
) -> impl Iterator<Item = (isize, isize)> {
    iter::repeat(()).enumerate().map(move |(i, _)| {
        (
            start.0 + i as isize * direction.0,
            start.1 + i as isize * direction.1,
        )
    })
}

pub fn walk_to_edge(
    start: (isize, isize),
    direction: (isize, isize),
    width: usize,
    height: usize,
) -> impl Iterator<Item = (isize, isize)> {
    walk(start, direction)
        .skip(1)
        .take_while(move |&(x, y)| x >= 0 && y >= 0 && x < width as isize && y < width as isize)
}

pub fn parse(inp: &str) -> (usize, usize, Vec<Vec<u8>>) {
    let res: Vec<Vec<_>> = inp
        .lines()
        .map(|row| row.chars().map(|i| i.to_digit(10).unwrap() as u8).collect())
        .collect();

    (res.len(), res[0].len(), res)
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_8() {
        run();
    }
}

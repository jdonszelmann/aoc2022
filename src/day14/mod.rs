pub mod part1;
pub mod part2;

use crate::day9::{add_pos, Pos};
use itertools::Itertools;
pub use part1::run as run_part1;
pub use part2::run as run_part2;
use std::collections::HashSet;
use std::hash::{BuildHasher, Hasher};

struct FastPairHash(u64);

impl FastPairHash {
    pub fn new() -> Self {
        Self(0)
    }
}

impl BuildHasher for FastPairHash {
    type Hasher = Self;

    fn build_hasher(&self) -> Self::Hasher {
        Self::new()
    }
}

impl Hasher for FastPairHash {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for i in bytes {
            self.0 ^= *i as u64;
            i.rotate_left(1);
        }
    }
}

pub fn draw_grid(walls: &HashSet<Pos>, sand: &HashSet<Pos>, floor: i64) {
    for y in 0..14 {
        for x in 488..=512 {
            if walls.contains(&(x, y)) || y == floor {
                print!("█");
            } else if sand.contains(&(x, y)) {
                print!("o");
            } else {
                print!(" ");
            }
        }
        println!()
    }
}

pub fn draw_line(line: Vec<Pos>, res: &mut HashSet<Pos>) {
    line.into_iter().tuple_windows().for_each(|(mut src, dst)| {
        let dx = (dst.0 - src.0).signum();
        let dy = (dst.1 - src.1).signum();

        while src != dst {
            res.insert(src);
            src = add_pos(src, (dx, dy));
        }
        res.insert(dst);
    });
}

pub fn draw_lines(lines: Vec<Vec<Pos>>) -> HashSet<Pos> {
    let mut res = HashSet::new();
    for line in lines {
        draw_line(line, &mut res);
    }
    res
}

const SAND_START: (i64, i64) = (500, 0);

pub fn drop_sand_grain(blocked: impl Fn(&Pos) -> bool) -> Pos {
    let mut curr = SAND_START;
    while let Some(new) = [(0, 1), (-1, 1), (1, 1)]
        .into_iter()
        .map(|i| add_pos(i, curr))
        .find(|x| !blocked(x))
    {
        curr = new;
    }

    curr
}

pub fn simulate(inp: &str, stop_on_floor: bool) -> usize {
    let lines = parse(inp);
    let floor = lines.iter().flatten().map(|i| i.1).max().unwrap() + 2;
    let walls = draw_lines(lines);
    // let mut sand = HashSet::with_hasher(FastPairHash::new());
    let mut sand = HashSet::new();

    loop {
        let i = drop_sand_grain(|p| walls.contains(p) || sand.contains(p) || p.1 == floor);

        if stop_on_floor && i.1 + 1 == floor {
            break;
        }

        sand.insert(i);

        if i == SAND_START {
            break;
        }
    }

    sand.len()
}

pub fn parse_line(inp: &str) -> Vec<Pos> {
    inp.split("->")
        .flat_map(|i| i.split(','))
        .map(|i| i.trim().parse().unwrap())
        .tuples()
        .collect()
}

pub fn parse(inp: &str) -> Vec<Vec<Pos>> {
    inp.lines().map(parse_line).collect()
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_14() {
        run();
    }
}

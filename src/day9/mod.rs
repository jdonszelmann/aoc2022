pub mod part1;
pub mod part2;

use itertools::Itertools;
pub use part1::run as run_part1;
pub use part2::run as run_part2;
use std::collections::HashSet;
use std::iter;

type Pos = (i64, i64);

pub fn add_pos(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

pub fn manhattan_dist_pos(a: Pos, b: Pos) -> Pos {
    ((a.0 - b.0).abs(), (a.1 - b.1).abs())
}

pub const AROUND: [Pos; 9] = [
    (0, 0),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

pub fn must_move(tail: Pos, head: Pos) -> bool {
    AROUND.iter().all(|a| add_pos(*a, tail) != head)
}

pub fn best_move(tail: Pos, head: Pos) -> Pos {
    AROUND[1..]
        .iter()
        .map(|i| add_pos(*i, tail))
        .min_by_key(|i| manhattan_dist_pos(*i, head))
        .unwrap()
}

pub type Snek = [Pos];

pub fn update_snek(snek: &mut Snek, head: Pos) {
    let [curr, rest@..] = snek else {
        return;
    };

    if must_move(*curr, head) {
        *curr = best_move(*curr, head);
    }

    update_snek(rest, *curr);
}

pub fn simulate_snek<const N: usize>(inp: &str) -> usize {
    let mut head = (0, 0);
    let mut snek = [(0, 0); N];
    let mut had = HashSet::new();

    for direction in parse(inp) {
        head = add_pos(head, direction);

        let mut prev = head;

        update_snek(&mut snek, head);
        had.insert(*snek.last().unwrap());
    }

    had.len()
}

pub fn parse(inp: &str) -> impl Iterator<Item = Pos> + '_ {
    inp.split([' ', '\n'])
        .tuples()
        .flat_map(|(dir, num)| iter::repeat(dir).take(num.parse().unwrap()))
        .map(|i| match i {
            "R" => (0, 1),
            "L" => (0, -1),
            "D" => (-1, 0),
            "U" => (1, 0),
            _ => unreachable!(),
        })
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_9() {
        run();
    }
}

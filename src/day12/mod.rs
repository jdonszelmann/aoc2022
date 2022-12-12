pub mod part1;
pub mod part2;

use crate::day9::{add_pos, Pos};
use itertools::iproduct;
use num::Zero;
pub use part1::run as run_part1;
pub use part2::run as run_part2;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use typed_arena::Arena;

pub fn cast_pos<T, U: TryFrom<T>>(pos: Pos<T>) -> Pos<U>
where
    <U as TryFrom<T>>::Error: Debug,
{
    (
        U::try_from(pos.0).unwrap(),
        U::try_from(pos.1).unwrap() as U,
    )
}

pub fn in_bounds<T: PartialOrd<T> + Zero>(pos: &Pos<T>, width: T, height: T) -> bool {
    pos.0 >= T::zero() && pos.1 >= T::zero() && pos.0 < width && pos.1 < height
}

pub fn height(name: char) -> usize {
    match name {
        'S' => 0,
        'a'..='z' => 1 + name as usize - 'a' as usize,
        'E' => 27,
        _ => panic!("bad height"),
    }
}

#[derive(Debug, Default)]
pub struct Node<'n> {
    parent: Option<&'n Node<'n>>,
    coord: Pos<usize>,
    name: char,
    cost: usize,
}

impl<'n> Node<'n> {
    pub fn can_move(&self, other: &Self) -> bool {
        height(other.name) <= height(self.name) + 1
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Pos<isize>> + '_ {
        [(-1isize, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|i| add_pos(i, cast_pos(self.coord)))
    }
}

impl Eq for Node<'_> {}

impl PartialEq<Self> for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Node<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub fn find_starts<'a: 'b, 'b>(
    inp: &'b [Vec<char>],
    starting_labels: &'b [char],
    arena: &'a Arena<Node<'a>>,
) -> impl Iterator<Item = &'a Node<'a>> + 'b {
    iproduct!(0..inp.len(), 0..inp[1].len())
        .filter(|&(y, x)| starting_labels.contains(&inp[y][x]))
        .map(|(y, x)| {
            &*arena.alloc(Node {
                coord: (x, y),
                name: inp[y][x],
                ..Default::default()
            })
        })
}

pub fn pathfind(inp: &str, starting_labels: &[char]) -> Option<usize> {
    let map = parse(inp);
    let width = map[0].len();
    let height = map.len();
    let alloc = Arena::new();

    let mut todo: BinaryHeap<&Node> = find_starts(&map, starting_labels, &alloc).collect();
    let mut had: HashMap<Pos<usize>, &Node> = Default::default();

    while let Some(curr) = todo.pop() {
        if had.contains_key(&curr.coord) {
            continue;
        }
        had.insert(curr.coord, curr);

        if curr.name == 'E' {
            return Some(curr.cost);
        }

        for pos in curr.neighbors() {
            if !in_bounds(&pos, width as isize, height as isize) || had.contains_key(&cast_pos(pos))
            {
                continue;
            }

            let new = alloc.alloc(Node {
                parent: Some(curr),
                coord: cast_pos(pos),
                name: map[pos.1 as usize][pos.0 as usize],
                cost: curr.cost + 1,
            });

            if curr.can_move(new) {
                todo.push(new);
            }
        }
    }

    None
}

pub fn parse(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|i| i.chars().collect()).collect()
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_12() {
        run();
    }
}

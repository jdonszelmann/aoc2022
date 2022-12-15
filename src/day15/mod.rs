pub mod part1;
pub mod part2;

use crate::day9::Pos;
use itertools::Itertools;
pub use part1::run as run_part1;
pub use part2::run as run_part2;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::ops::Range;

pub fn manhattan_dist_pos_added(a: Pos<i64>, b: Pos<i64>) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

struct RangeSet {
    ranges: HashSet<Range<i64>>,
}

impl Debug for RangeSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.ranges.iter().map(|i| format!("{:?}", i)).join(", ")
        )
    }
}

impl RangeSet {
    pub fn new() -> Self {
        Self {
            ranges: Default::default(),
        }
    }

    fn overlaps(a: &Range<i64>, b: &Range<i64>) -> bool {
        // a.contains(&b.start) || b.contains(&a.start)
        (b.start <= a.start && a.start <= b.end) || (a.start <= b.start && b.start <= a.end)
    }

    fn merge(a: &Range<i64>, b: &Range<i64>) -> Range<i64> {
        a.start.min(b.start)..a.end.max(b.end)
    }

    pub fn add(&mut self, range: Range<i64>) {
        let mut curr = range;
        loop {
            let mut to_remove = None;

            for i in &self.ranges {
                if Self::overlaps(i, &curr) {
                    to_remove = Some(i.clone());
                    break;
                }
            }

            if let Some(i) = to_remove {
                self.ranges.remove(&i);
                curr = Self::merge(&curr, &i);
            } else {
                break;
            }
        }

        self.ranges.insert(curr);
    }

    pub fn contains(&self, coord: i64) -> bool {
        self.ranges
            .iter()
            .any(|i| Self::overlaps(i, &(coord..coord)))
    }

    pub fn len(&self) -> usize {
        self.ranges.iter().map(|i| (i.end - i.start) as usize).sum()
    }
}

pub fn range_at_y(sensor: Pos, beacon: Pos, y: u64, limit: Option<u64>) -> Option<Range<i64>> {
    let dist = manhattan_dist_pos_added(sensor, beacon);
    let dist_to_critical = manhattan_dist_pos_added(sensor, (sensor.0, y as i64));
    if dist_to_critical <= dist {
        let dist_crit_beacon = dist - dist_to_critical;
        let mut start = sensor.0 - dist_crit_beacon;
        let mut end = sensor.0 + dist_crit_beacon;
        if let Some(l) = limit {
            start = start.max(0);
            end = end.min(l as i64);
        }

        Some(start..end)
    } else {
        None
    }
}

pub fn parse(inp: &str) -> impl Iterator<Item = (Pos, Pos)> + '_ {
    let regex = Regex::new(
        r"Sensor at x=([\-0-9]+), y=([\-0-9]+): closest beacon is at x=([\-0-9]+), y=([\-0-9]+)",
    )
    .unwrap();

    inp.lines()
        .flat_map(move |i| {
            regex
                .captures(i)
                .unwrap()
                .iter()
                .skip(1)
                .collect::<Vec<_>>()
        })
        .map(|i| i.unwrap().as_str().parse::<i64>().unwrap())
        .tuples()
        .tuples()
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_15() {
        run();
    }
}

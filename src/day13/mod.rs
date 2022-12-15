pub mod part1;
pub mod part2;

use crate::day11::Parser;
use crate::day13::IntOrList::{Int, List};
use itertools::Itertools;
pub use part1::run as run_part1;
pub use part2::run as run_part2;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum IntOrList {
    Int(u64),
    List(Vec<IntOrList>),
}

impl Ord for IntOrList {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Int(a), Int(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (Int(a), b) => List(vec![Int(*a)]).cmp(b),
            (a, Int(b)) => a.cmp(&List(vec![Int(*b)])),
        }
    }
}

impl Eq for IntOrList {}

impl PartialEq<Self> for IntOrList {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for IntOrList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for IntOrList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Int(i) => write!(f, "{i}"),
            List(i) => write!(f, "[{}]", i.iter().map(ToString::to_string).join(",")),
        }
    }
}

impl IntOrList {
    fn parse_expr(stream: &mut Parser) -> Option<IntOrList> {
        if stream.is_empty() {
            return None;
        }

        stream.whitespace();
        if let Some(i) = stream.parse_num() {
            Some(Int(i))
        } else {
            Self::parse_list(stream).map(List)
        }
    }

    pub fn parse_list(stream: &mut Parser) -> Option<Vec<IntOrList>> {
        let mut elems = Vec::new();
        stream.whitespace();
        stream.accept('[');
        loop {
            stream.whitespace();
            if stream.accept(']').is_some() {
                return Some(elems);
            } else {
                elems.push(Self::parse_expr(stream)?);
                stream.whitespace();
                // accept if we can, but don't fail if we don't
                let _ = stream.accept(',');
                stream.whitespace();
            }
        }
    }
}

pub fn parse_list(inp: &str) -> IntOrList {
    IntOrList::parse_expr(&mut Parser::new(inp)).unwrap()
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_13() {
        run();
    }
}

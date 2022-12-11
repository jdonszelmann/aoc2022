pub mod part1;
pub mod part2;

use itertools::{Itertools, MultiPeek};
use minmaxn::MinMaxN;
pub use part1::run as run_part1;
pub use part2::run as run_part2;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
pub enum Operation {
    Old,
    Mul(Box<Self>, Box<Self>),
    Add(Box<Self>, Box<Self>),
    Const(i64),
}

impl Operation {
    pub fn eval(&self, old: i64) -> i64 {
        match self {
            Operation::Old => old,
            Operation::Mul(a, b) => a.eval(old) * b.eval(old),
            Operation::Add(a, b) => a.eval(old) + b.eval(old),
            Operation::Const(c) => *c,
        }
    }
}

type OpFn<S> = fn(Box<S>, Box<S>) -> S;

impl Operation {
    fn whitespace(stream: &mut MultiPeek<impl Iterator<Item = char>>) {
        if let Some(i) = stream.peek() {
            if i.is_whitespace() {
                stream.next();
                Self::whitespace(stream);
            }
        }

        stream.reset_peek();
    }

    fn parse_expr(stream: &mut MultiPeek<impl Iterator<Item = char>>) -> Option<Self> {
        Self::whitespace(stream);
        let left = Self::parse_atom(stream)?;
        Self::whitespace(stream);

        if let Some(op) = Self::parse_op(stream) {
            Self::whitespace(stream);
            Some(op(Box::new(left), Box::new(Self::parse_atom(stream)?)))
        } else {
            Some(left)
        }
    }

    fn parse_op(stream: &mut MultiPeek<impl Iterator<Item = char>>) -> Option<OpFn<Self>> {
        match stream.peek() {
            Some('*') => {
                stream.next();
                Some(Self::Mul)
            }
            Some('+') => {
                stream.next();
                Some(Self::Add)
            }
            _ => {
                stream.reset_peek();
                None
            }
        }
    }

    fn parse_atom(stream: &mut MultiPeek<impl Iterator<Item = char>>) -> Option<Self> {
        Self::parse_old(stream).or_else(|| Self::parse_const(stream))
    }

    fn parse_const(stream: &mut MultiPeek<impl Iterator<Item = char>>) -> Option<Self> {
        let mut res = String::new();
        while let Some(i) = stream.peek() {
            if i.is_ascii_digit() {
                res.push(*i)
            } else {
                stream.reset_peek();
                break;
            }
        }

        if res.is_empty() {
            None
        } else {
            Some(Self::Const(res.parse().ok()?))
        }
    }

    fn parse_old(stream: &mut MultiPeek<impl Iterator<Item = char>>) -> Option<Self> {
        if stream.peek() == Some(&'o') && stream.peek() == Some(&'l') && stream.peek() == Some(&'d')
        {
            for i in 0..3 {
                stream.next();
            }
            Some(Self::Old)
        } else {
            stream.reset_peek();
            None
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_expr(&mut s.chars().multipeek()).ok_or(())
    }
}

#[derive(Debug)]
pub struct Monkey {
    num: usize,
    items: RefCell<VecDeque<i64>>,
    operation: Operation,
    test: i64,
    if_true: usize,
    if_false: usize,
    num_inspections: Cell<u64>,
}

pub fn parse(inp: &str) -> Vec<Monkey> {
    inp.split("\n\n")
        .flat_map(|i| i.split('\n'))
        .map(str::trim)
        .tuples()
        .map(
            |(monkey, starting, operation, test, if_true, if_false)| Monkey {
                num: monkey
                    .strip_prefix("Monkey ")
                    .unwrap()
                    .strip_suffix(':')
                    .unwrap()
                    .parse()
                    .unwrap(),
                items: RefCell::new(
                    starting
                        .strip_prefix("Starting items: ")
                        .unwrap()
                        .split(", ")
                        .map(|i| i.parse().unwrap())
                        .collect(),
                ),
                operation: operation
                    .strip_prefix("Operation: new = ")
                    .unwrap()
                    .parse()
                    .unwrap(),
                test: test
                    .strip_prefix("Test: divisible by ")
                    .unwrap()
                    .parse()
                    .unwrap(),
                if_true: if_true
                    .strip_prefix("If true: throw to monkey ")
                    .unwrap()
                    .parse()
                    .unwrap(),
                if_false: if_false
                    .strip_prefix("If false: throw to monkey ")
                    .unwrap()
                    .parse()
                    .unwrap(),
                num_inspections: Cell::new(0),
            },
        )
        .collect()
}

pub fn simulate(inp: &str, rounds: usize, divide: bool) -> u64 {
    let monkeys = parse(inp);

    let multiple = monkeys.iter().map(|i| i.test).product::<i64>();

    for r in 0..rounds {
        for monkey in &monkeys {
            while let Some(i) = monkey.items.borrow_mut().pop_front() {
                let worry_level = monkey.operation.eval(i);
                let reduced_worry_level = if divide {
                    worry_level / 3
                } else {
                    worry_level % multiple
                };

                if reduced_worry_level % monkey.test == 0 {
                    monkeys[monkey.if_true]
                        .items
                        .borrow_mut()
                        .push_back(reduced_worry_level);
                } else {
                    monkeys[monkey.if_false]
                        .items
                        .borrow_mut()
                        .push_back(reduced_worry_level);
                }

                monkey.num_inspections.set(monkey.num_inspections.get() + 1);
            }
        }
    }

    monkeys
        .into_iter()
        .map(|i| i.num_inspections.get())
        .max_n_all::<2>()
        .unwrap()
        .into_iter()
        .product()
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_11() {
        run();
    }
}

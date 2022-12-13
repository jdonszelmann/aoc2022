pub mod part1;
pub mod part2;

use itertools::{Itertools, MultiPeek};
use minmaxn::MinMaxN;
pub use part1::run as run_part1;
pub use part2::run as run_part2;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::str::{Chars, FromStr};

pub struct Parser<'a> {
    stream: MultiPeek<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(i: &'a str) -> Self {
        Self {
            stream: i.chars().multipeek(),
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.stream.peek().is_none()
    }

    pub fn accept(&mut self, c: char) -> Option<()> {
        if self.stream.peek() == Some(&c) {
            let _ = self.stream.next();
            // println!("accept: {c}");
            Some(())
        } else {
            self.stream.reset_peek();
            None
        }
    }

    pub fn accept_with(&mut self, c: impl Fn(char) -> bool) -> Option<char> {
        if let Some(&i) = self.stream.peek() {
            if c(i) {
                let _ = self.stream.next();
                Some(i)
            } else {
                self.stream.reset_peek();
                None
            }
        } else {
            self.stream.reset_peek();
            None
        }
    }

    pub fn accept_str(&mut self, s: &str) -> Option<()> {
        for i in s.chars() {
            if self.stream.peek() != Some(&i) {
                self.stream.reset_peek();
                return None;
            }
        }

        // println!("accept: {s}");
        for i in s.chars() {
            self.stream.next();
        }

        Some(())
    }

    pub fn whitespace(&mut self) -> bool {
        let mut res = false;

        while self.accept_with(|i| i.is_whitespace()).is_some() {
            res = true;
        }

        res
    }

    pub fn parse_num<T: FromStr>(&mut self) -> Option<T> {
        let mut res = String::new();

        while let Some(i) = self.accept_with(|i| i.is_ascii_digit()) {
            res.push(i)
        }

        if res.is_empty() {
            None
        } else {
            // println!("num: {res}");
            Some(res.parse().ok()?)
        }
    }
}

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
    fn parse_expr(stream: &mut Parser) -> Option<Self> {
        stream.whitespace();
        let left = Self::parse_atom(stream)?;
        stream.whitespace();

        if let Some(op) = Self::parse_op(stream) {
            stream.whitespace();
            Some(op(Box::new(left), Box::new(Self::parse_atom(stream)?)))
        } else {
            Some(left)
        }
    }

    fn parse_op(stream: &mut Parser) -> Option<OpFn<Self>> {
        if stream.accept('*').is_some() {
            Some(Self::Mul)
        } else if stream.accept('+').is_some() {
            Some(Self::Add)
        } else {
            None
        }
    }

    fn parse_atom(stream: &mut Parser) -> Option<Self> {
        Self::parse_old(stream).or_else(|| Self::parse_const(stream))
    }

    fn parse_const(stream: &mut Parser) -> Option<Self> {
        Some(Self::Const(stream.parse_num()?))
    }

    fn parse_old(stream: &mut Parser) -> Option<Self> {
        stream.accept_str("old").map(|_| Self::Old)
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

impl Monkey {
    fn parse_header(stream: &mut Parser) -> Option<usize> {
        stream.whitespace();
        stream.accept_str("Monkey")?;
        stream.whitespace();
        let num = stream.parse_num()?;
        stream.accept(':')?;
        stream.whitespace();

        Some(num)
    }

    fn parse_starting_items(stream: &mut Parser) -> Option<VecDeque<i64>> {
        stream.whitespace();
        stream.accept_str("Starting")?;
        stream.whitespace();
        stream.accept_str("items")?;
        stream.whitespace();
        stream.accept(':')?;
        stream.whitespace();

        let mut res = VecDeque::new();

        loop {
            res.push_back(stream.parse_num()?);

            if stream.accept_str(", ").is_none() {
                break;
            }
        }

        Some(res)
    }

    fn parse_operation(stream: &mut Parser) -> Option<Operation> {
        stream.whitespace();
        stream.accept_str("Operation")?;
        stream.whitespace();
        stream.accept(':')?;
        stream.whitespace();
        stream.accept_str("new")?;
        stream.whitespace();
        stream.accept_str("=")?;
        stream.whitespace();

        Operation::parse_expr(stream)
    }

    fn parse_test(stream: &mut Parser) -> Option<i64> {
        stream.whitespace();
        stream.accept_str("Test")?;
        stream.whitespace();
        stream.accept(':')?;
        stream.whitespace();
        stream.accept_str("divisible")?;
        stream.whitespace();
        stream.accept_str("by")?;
        stream.whitespace();

        stream.parse_num()
    }

    fn parse_condition(stream: &mut Parser, condition: &str) -> Option<usize> {
        stream.whitespace();
        stream.accept_str("If")?;
        stream.whitespace();
        stream.accept_str(condition)?;
        stream.whitespace();
        stream.accept(':')?;
        stream.whitespace();

        Self::parse_throw(stream)
    }

    fn parse_throw(stream: &mut Parser) -> Option<usize> {
        stream.whitespace();
        stream.accept_str("throw")?;
        stream.whitespace();
        stream.accept_str("to")?;
        stream.whitespace();
        stream.accept_str("monkey")?;
        stream.whitespace();

        stream.parse_num()
    }

    fn parse_monkey(stream: &mut Parser) -> Option<Self> {
        Some(Self {
            num: Self::parse_header(stream)?,
            items: RefCell::new(Self::parse_starting_items(stream)?),
            operation: Self::parse_operation(stream)?,
            test: Self::parse_test(stream)?,
            if_true: Self::parse_condition(stream, "true")?,
            if_false: Self::parse_condition(stream, "false")?,
            num_inspections: Cell::new(0),
        })
    }
}

pub fn parse(inp: &str) -> Vec<Monkey> {
    inp.split("\n\n")
        .map(|i| Monkey::parse_monkey(&mut Parser::new(i)).unwrap())
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

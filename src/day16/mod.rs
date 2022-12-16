pub mod part1;
pub mod part2;

use itertools::Itertools;
pub use part1::run as run_part1;
pub use part2::run as run_part2;
use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;
use typed_arena::Arena;

#[derive(Clone, Debug)]
pub struct Node {
    rate: u8,
    conns: Vec<u8>,
}

pub fn parse(inp: &str) -> (Vec<Node>, u8) {
    let regex =
        Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();

    let names: HashMap<&str, u8> = HashMap::new();
    let name_ctr = 0;

    let nodes = inp
        .lines()
        .flat_map(move |i| {
            regex
                .captures(i)
                .unwrap()
                .iter()
                .skip(1)
                .collect::<Vec<_>>()
        })
        .tuples()
        .map(|(name, flow, conns)| {
            (
                name.unwrap().as_str(),
                flow.unwrap().as_str().parse::<u8>().unwrap(),
                conns.unwrap().as_str().split(", ").collect::<Vec<_>>(),
            )
        })
        .collect_vec();

    let mut hash_res = HashMap::new();
    let mut names = HashMap::new();
    let mut name_ctr = 0u8;
    for (name, flow, conns) in &nodes {
        let num_name = names.entry(*name).or_insert_with(|| {
            let old = name_ctr;
            name_ctr += 1;
            old
        });

        hash_res.insert(
            *num_name,
            Node {
                rate: *flow,
                conns: vec![],
            },
        );
    }

    for (name, _, conns) in nodes {
        hash_res.get_mut(names.get(name).unwrap()).unwrap().conns =
            conns.into_iter().map(|i| *names.get(&i).unwrap()).collect();
    }

    let mut res = Vec::new();
    for i in 0..hash_res.len() as u8 {
        res.push(hash_res.get(&i).unwrap().clone());
    }
    (res, *names.get(&"AA").unwrap() as u8)
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_16() {
        run();
    }
}

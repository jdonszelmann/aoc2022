use crate::day16::parse;
use crate::day16::Node;
use memoize::memoize;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 16 part 1");

    let contents = read_to_string("src/day16/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn search(
    nodes: &Vec<Node>,
    curr: u8,
    opened: &HashSet<u8>,
    flow_rate: u64,
    current_total: u64,
    time: u8,
    cache: &mut HashMap<(u8, u64, u8), u64>,
) -> Option<u64> {
    if time > 30 {
        return Some(current_total);
    }

    if let Some(&i) = cache.get(&(curr, flow_rate, time)) {
        if i >= current_total {
            return None;
        }
    }
    cache.insert((curr, flow_rate, time), current_total);

    let mut max = None;
    let curr_node = &nodes[curr as usize];

    if !opened.contains(&curr) && curr_node.rate > 0 {
        let mut new_opened = opened.clone();
        new_opened.insert(curr);

        let new_flow_rate = flow_rate + curr_node.rate as u64;

        max = max.max(search(
            nodes,
            curr,
            &new_opened,
            new_flow_rate,
            current_total + flow_rate,
            time + 1,
            cache,
        ));
    }

    for &i in &curr_node.conns {
        max = max.max(search(
            nodes,
            i,
            opened,
            flow_rate,
            current_total + flow_rate,
            time + 1,
            cache,
        ));
    }

    max
}

pub fn implementation(inp: &str) -> u64 {
    let (inp, s) = parse(inp);
    let mut cache = HashMap::new();

    search(&inp, s, &HashSet::new(), 0, 0, 1, &mut cache).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day16::part1::implementation;
    use std::fs::read_to_string;

    #[test]
    #[ignore]
    pub fn test_day_16_part_1() {
        let contents = read_to_string("src/day16/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 1796);
    }

    #[test]
    #[ignore]
    pub fn test_day_16_part_1_test_input() {
        let contents = read_to_string("src/day16/data.test").expect("no input file found");
        assert_eq!(implementation(&contents), 1651);
    }
}

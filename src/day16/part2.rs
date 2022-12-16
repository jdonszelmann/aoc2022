use crate::day16::parse;
use crate::day16::Node;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 16 part 2");

    let contents = read_to_string("src/day16/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

#[allow(clippy::too_many_arguments)]
pub fn search(
    nodes: &Vec<Node>,
    you: u8,
    elephant: u8,
    opened: &HashSet<u8>,
    flow_rate: u64,
    current_total: u64,
    time: u8,
    cache: &mut HashMap<(u8, u8, u64, u8), u64>,
) -> Option<u64> {
    if time > 26 {
        return Some(current_total);
    }

    if let Some(&i) = cache.get(&(you, elephant, flow_rate, time)) {
        if i >= current_total {
            return None;
        }
    }
    cache.insert((you, elephant, flow_rate, time), current_total);

    let mut max = None;
    let your_curr_node = &nodes[you as usize];
    let elephant_curr_node = &nodes[elephant as usize];

    // I open, elephant moves
    if !opened.contains(&you) && your_curr_node.rate > 0 {
        let mut new_opened = opened.clone();
        new_opened.insert(you);

        let new_flow_rate = flow_rate + your_curr_node.rate as u64;

        for &i in &elephant_curr_node.conns {
            max = max.max(search(
                nodes,
                you,
                i,
                &new_opened,
                new_flow_rate,
                current_total + flow_rate,
                time + 1,
                cache,
            ));
        }
    }
    // Elephant opens, I move
    if !opened.contains(&elephant) && elephant_curr_node.rate > 0 {
        let mut new_opened = opened.clone();
        new_opened.insert(elephant);

        let new_flow_rate = flow_rate + elephant_curr_node.rate as u64;

        for &i in &your_curr_node.conns {
            max = max.max(search(
                nodes,
                i,
                elephant,
                &new_opened,
                new_flow_rate,
                current_total + flow_rate,
                time + 1,
                cache,
            ));
        }
    }
    // we both open
    if !opened.contains(&elephant)
        && !opened.contains(&you)
        && elephant_curr_node.rate > 0
        && your_curr_node.rate > 0
        && elephant != you
    {
        let mut new_opened = opened.clone();
        new_opened.insert(you);
        new_opened.insert(elephant);

        let new_flow_rate = flow_rate + your_curr_node.rate as u64 + elephant_curr_node.rate as u64;

        max = max.max(search(
            nodes,
            you,
            elephant,
            &new_opened,
            new_flow_rate,
            current_total + flow_rate,
            time + 1,
            cache,
        ));
    }

    for &i in &your_curr_node.conns {
        for &j in &elephant_curr_node.conns {
            max = max.max(search(
                nodes,
                i,
                j,
                opened,
                flow_rate,
                current_total + flow_rate,
                time + 1,
                cache,
            ));
        }
    }

    max
}

pub fn implementation(inp: &str) -> u64 {
    let (inp, s) = parse(inp);
    let mut cache = HashMap::new();

    search(&inp, s, s, &HashSet::new(), 0, 0, 1, &mut cache).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day16::part2::implementation;
    use std::fs::read_to_string;

    #[test]
    #[ignore]
    pub fn test_day_16_part_2() {
        let contents = read_to_string("src/day16/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 1999);
    }

    #[test]
    #[ignore]
    pub fn test_day_16_part_2_test_input() {
        let contents = read_to_string("src/day16/data.test").expect("no input file found");
        assert_eq!(implementation(&contents), 1707);
    }
}

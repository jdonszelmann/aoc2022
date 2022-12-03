use crate::day3::parse;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 3 part 2");

    let contents = read_to_string("src/day3/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> u64 {
    parse(inp)
        .chunks(3)
        .into_iter()
        .flat_map(|i| i.into_iter()
            .map(|i| i.chars())
            .map(HashSet::from_iter)
            .reduce(|ref x, ref y| x.intersection(y).copied().collect::<HashSet<_>>())
            .unwrap()
        )
        .map(|i| match i {
            'a'..='z' => i as u64 - 97,
            'A'..='Z' => i as u64 - 39,
            _ => unreachable!(),
        } + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day3::part2::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_3_part_2() {
        let contents = read_to_string("src/day3/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 2758)
    }

    #[test]
    pub fn test_day_3_part_2_test_input() {
        let testdata = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(implementation(testdata), 70)
    }
}

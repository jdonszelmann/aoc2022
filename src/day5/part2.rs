use crate::day5::parse;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 5 part 2");

    let contents = read_to_string("src/day5/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> String {
    let (sequence, mut stacks) = parse(inp);
    let mut tmp = Vec::new();

    for (num, src, dst) in sequence {
        for _ in 0..num {
            tmp.push(stacks[src - 1].pop().unwrap())
        }

        stacks[dst - 1].extend(tmp.drain(..).rev())
    }

    stacks.into_iter().map(|i| *i.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use crate::day5::part2::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_5_part_2() {
        let contents = read_to_string("src/day5/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), "DCVTCVPCL")
    }

    #[test]
    pub fn test_day_5_part_2_test_input() {
        let testdata = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(implementation(testdata), "MCD");
    }
}

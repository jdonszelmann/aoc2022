use crate::day7::parse;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 7 part 1");

    let contents = read_to_string("src/day7/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> usize {
    let mut dirsizes = Vec::new();
    parse(inp).stats(&mut dirsizes);

    dirsizes.into_iter().filter(|&i| i < 100_000).sum()
}

#[cfg(test)]
mod tests {
    use crate::day7::part1::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_7_part_1() {
        let contents = read_to_string("src/day7/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 1334506)
    }

    #[test]
    pub fn test_day_7_part_1_test_input() {
        let testdata = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

        assert_eq!(implementation(testdata), 95437);
    }
}

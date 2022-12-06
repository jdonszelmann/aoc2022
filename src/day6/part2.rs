use crate::day6::first_subsequence_length;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 6 part 2");

    let contents = read_to_string("src/day6/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> usize {
    first_subsequence_length::<_, 14>(inp.chars())
}

#[cfg(test)]
mod tests {
    use crate::day6::part2::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_6_part_2() {
        let contents = read_to_string("src/day6/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 3588)
    }

    #[test]
    pub fn test_day_6_part_2_test_input() {
        assert_eq!(implementation("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(implementation("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(implementation("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(implementation("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(implementation("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}

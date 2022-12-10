use crate::day10::run_cpu;
use crate::day2::parse;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 10 part 2");

    let contents = read_to_string("src/day10/data.in").expect("no input file found");
    println!("{}", implementation(&contents).join("\n"));
}

pub fn implementation(inp: &str) -> Vec<String> {
    let mut total: Vec<Vec<char>> = vec![vec![' '; 40]; 6];
    run_cpu(inp, |x: usize, y: usize, c, sprite_x, hit| {
        if hit {
            total[y][x] = '█';
        }
    });

    total.into_iter().map(|i| i.into_iter().collect()).collect()
}

#[cfg(test)]
mod tests {
    use crate::day10::part2::implementation;
    use std::fs::read_to_string;

    fn parse_output(out: &str) -> Vec<String> {
        out.trim()
            .replace('.', " ")
            .replace('#', "█")
            .lines()
            .map(ToString::to_string)
            .collect()
    }

    #[test]
    pub fn test_day_10_part_2() {
        let contents = read_to_string("src/day10/data.in").expect("no input file found");
        assert_eq!(
            implementation(&contents),
            parse_output(
                "
###..####..##..###..#..#.###..####.###..
#..#....#.#..#.#..#.#.#..#..#.#....#..#.
#..#...#..#....#..#.##...#..#.###..###..
###...#...#.##.###..#.#..###..#....#..#.
#....#....#..#.#....#.#..#....#....#..#.
#....####..###.#....#..#.#....####.###..
        "
            )
        )
    }

    #[test]
    pub fn test_day_10_part_2_test_input() {
        let contents = read_to_string("src/day10/2.test").expect("no input file found");
        assert_eq!(
            implementation(&contents),
            parse_output(
                "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            )
        );
    }
}

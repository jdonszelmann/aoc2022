pub mod part1;
pub mod part2;

use itertools::Itertools;
pub use part1::run as run_part1;
pub use part2::run as run_part2;

// not mine, https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn parse(
    inp: &str,
) -> (
    impl Iterator<Item = (usize, usize, usize)> + '_,
    Vec<Vec<char>>,
) {
    let [start, sequence]: [&str; 2] = inp.split("\n\n").collect::<Vec<_>>().try_into().unwrap();

    let table: Vec<Vec<String>> = start
        .split('\n')
        .filter(|i| i.chars().any(|i| i.is_alphabetic()))
        .map(|i| {
            i.replace("    ", " ")
                .split(' ')
                .map(ToString::to_string)
                .collect()
        })
        .collect();

    let longest = table.iter().map(|i| i.len()).max().unwrap();
    let extended_table = table
        .into_iter()
        .map(|mut i| {
            i.resize(longest, "".to_string());
            i
        })
        .collect();

    let stacks = transpose(extended_table)
        .into_iter()
        .map(|i| {
            i.into_iter()
                .filter(|i| !i.is_empty())
                .map(|i| i.chars().nth(1).unwrap())
                .rev()
                .collect()
        })
        .collect();

    let parsed_sequence = sequence
        .split('\n')
        .map(|i| {
            i.split(' ')
                .map(|i| i.parse::<usize>())
                .filter_map(Result::ok)
                .collect_tuple()
        })
        .map(|i| i.unwrap());

    (parsed_sequence, stacks)
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_5() {
        run();
    }
}

import os
from pathlib import Path

dirs_path = Path(os.path.realpath(__file__)).parent / "src"


def main():
    os.makedirs(dirs_path)

    if not (dirs_path / "main.rs").exists():
        with open(dirs_path / "main.rs", "w") as f:
            contents = "\n".join(f'mod day{i};' for i in range(1, 26))
            runs = "\n    ".join(f'day{i}::run();' for i in range(1, 26))

            f.write(f"""
{contents}

fn main() {{
    {runs}
}}
                """)

    for i in range(1, 26):
        d = dirs_path / f"day{i}"
        os.makedirs(d)

        if not (d / "input_part1.txt").exists():
            with open(d / "input_part1.txt", "w") as f:
                pass
        if not (d / "input_part2.txt").exists():
            with open(d / "input_part2.txt", "w") as f:
                pass

        if not (d / "main.py").exists():
            with open(d / "main.py", "w") as f:
                f.write("""
def main():
    input_part1 = open("input_part1.txt").read()
    input_part2 = open("input_part2.txt").read()
                
if __name__ == '__main__':
    main()  
                """)

        if not (d / "mod.rs").exists():
            with open(d / "mod.rs", "w") as f:
                f.write(f"""
pub mod part1;
pub mod part2;

pub use part1::run as run_part1;
pub use part2::run as run_part2;

pub fn run() {{
    run_part1();
    run_part2();
}}

#[cfg(test)]
mod tests {{
    use super::run;

    #[test]
    pub fn test_day_{i}() {{
        run();
    }}
}}

                            """)

            with open(d / "part1.rs", "w") as f:
                f.write(f"""
use std::fs::read;
                
pub fn run() {{
    println!("aoc 2022 day {i} part 1");

    let input = read("{d}/input_part1.txt").expect("no input file found");
}}

#[cfg(test)]
mod tests {{
    use super::run;

    #[test]
    pub fn test_day_{i}_part_1() {{
        run();
    }}
}}
        """)

                with open(d / "part2.rs", "w") as f:
                    f.write(f"""
use std::fs::read;

pub fn run() {{
    println!("aoc 2022 day {i} part 2");

    let input = read("{d}/input_part1.txt").expect("no input file found");
}}


#[cfg(test)]
mod tests {{
    use super::run;
    
    #[test]
    pub fn test_day_{i}_part_2() {{
        run();
    }}
}}
                    """)


if __name__ == '__main__':
    main()

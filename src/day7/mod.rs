pub mod part1;
pub mod part2;

pub use part1::run as run_part1;
pub use part2::run as run_part2;
use std::collections::HashMap;

#[derive(Default)]
pub struct Files<'a>(HashMap<&'a str, File<'a>>);

impl<'a, 'b> IntoIterator for &'b Files<'a> {
    type Item = &'b File<'a>;
    type IntoIter = std::collections::hash_map::Values<'b, &'a str, File<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.values()
    }
}

impl<'a> Files<'a> {
    pub fn find_dir(&mut self, path: &[&str]) -> Option<&mut Files<'a>> {
        if path.is_empty() {
            return Some(self);
        }

        let File::Dir(file) = self.0.get_mut(path[0])? else {
            return None;
        };

        file.find_dir(&path[1..])
    }

    pub fn create(&mut self, name: &'a str, file: File<'a>) {
        self.0.insert(name, file);
    }

    pub fn stats(&self, dirsizes: &mut Vec<usize>) -> usize {
        let mut total = 0;

        for i in self {
            match i {
                File::Dir(d) => {
                    let size = d.stats(dirsizes);
                    dirsizes.push(size);
                    total += size;
                }
                File::File(size) => {
                    total += size;
                }
            }
        }

        total
    }
}

pub enum File<'a> {
    Dir(Files<'a>),
    File(usize),
}

pub fn parse<'a>(inp: &'a str) -> Files<'a> {
    let mut root = Files::default();
    let mut path: Vec<&'a str> = vec![];

    for i in inp.lines() {
        match *i.split(' ').collect::<Vec<_>>().as_slice() {
            ["$", "ls", ..] => {}
            ["$", "cd", "/"] => {}
            ["$", "cd", ".."] => {
                path.pop();
            }
            ["$", "cd", name] => {
                path.push(name);
            }
            ["dir", name] => root
                .find_dir(&path)
                .unwrap()
                .create(name, File::Dir(Default::default())),
            [size, name] => {
                root.find_dir(&path)
                    .unwrap()
                    .create(name, File::File(size.parse().unwrap()));
            }
            _ => unreachable!(),
        }
    }

    root
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_7() {
        run();
    }
}

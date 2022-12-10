pub mod part1;
pub mod part2;

pub use part1::run as run_part1;
pub use part2::run as run_part2;

pub fn parse(inp: &str) -> impl Iterator<Item = Vec<&str>> + '_ {
    inp.lines().map(|i| i.split(' ').collect())
}

pub fn run_cpu(inp: &str, mut handler: impl CycleHandler) {
    let mut sprite_x: i64 = 1;

    let mut cycles = 0;
    let mut beam_y = 0;

    let mut inc_cycles = |sprite_x| {
        let beam_x = cycles % 40;
        cycles += 1;

        let sprite_hit = (sprite_x - 1..=sprite_x + 1).contains(&(beam_x as i64));
        handler.call(beam_x, beam_y, cycles, sprite_x, sprite_hit);

        if beam_x == 39 {
            beam_y = (beam_y + 1) % 6
        }
    };

    for i in parse(inp) {
        match i.as_slice() {
            ["noop"] => {
                inc_cycles(sprite_x);
            }
            ["addx", a] => {
                inc_cycles(sprite_x);
                inc_cycles(sprite_x);
                sprite_x += a.parse::<i64>().unwrap();
            }
            _ => unreachable!(),
        }
    }
}

pub trait CycleHandler {
    fn call(&mut self, x: usize, y: usize, cycles: usize, sprite_x: i64, sprite_hit: bool);
}

impl<T> CycleHandler for T
where
    T: FnMut(usize, usize, usize, i64, bool),
{
    fn call(&mut self, x: usize, y: usize, cycles: usize, sprite_x: i64, sprite_hit: bool) {
        (self)(x, y, cycles, sprite_x, sprite_hit)
    }
}

pub fn run() {
    run_part1();
    run_part2();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_day_10() {
        run();
    }
}

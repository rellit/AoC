use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum Rock {
    Stable,
    Movable,
}

#[derive(Clone, Debug)]
struct Mirror {
    w: usize,
    h: usize,
    rocks: HashMap<(usize, usize), Rock>,
}

impl Mirror {
    fn _print(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                match self.rocks.get(&(x, y)) {
                    Some(Rock::Movable) => print!("O"),
                    Some(Rock::Stable) => print!("#"),
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let mut mirror = parse_input(input);

    mirror = north(mirror);

    value(&mirror)
}

fn value(mirror: &Mirror) -> usize {
    let mut res = 0;

    (0..mirror.h).rev().enumerate().for_each(|(i, y)| {
        (0..mirror.w).for_each(|x| {
            if let Some(Rock::Movable) = mirror.rocks.get(&(x, y)) {
                res += i + 1;
            }
        });
    });

    res
}

fn north(mut mirror: Mirror) -> Mirror {
    (0..mirror.w).for_each(|x| {
        let mut target_pos = None;
        (0..mirror.h).for_each(|y| match mirror.rocks.get(&(x, y)) {
            None => {
                if target_pos.is_none() {
                    target_pos = Some((x, y))
                }
            }

            Some(Rock::Movable) => match target_pos {
                None => {}
                Some((x_target, y_target)) => {
                    mirror.rocks.insert((x_target, y_target), Rock::Movable);
                    mirror.rocks.remove(&(x, y));
                    target_pos = Some((x, y.min(y_target + 1)));
                }
            },
            Some(Rock::Stable) => target_pos = None,
        });
    });

    mirror
}

fn parse_input(input: &str) -> Mirror {
    let mut w = 0;
    let mut h = 0;
    let mut rocks = HashMap::new();
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            match c {
                '#' => rocks.insert((x, y), Rock::Stable),
                'O' => rocks.insert((x, y), Rock::Movable),
                '.' => None,
                _ => panic!("Unknown input"),
            };
            w = w.max(x + 1);
            h = h.max(y + 1);
        })
    });

    Mirror { rocks, w, h }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        let result = solve(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        assert_eq!(result, 136);
    }
}

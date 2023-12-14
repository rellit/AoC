use std::collections::HashMap;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator, IntoParallelIterator};

#[derive(Clone, Debug)]
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
    fn print(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                match self.rocks.get(&(x,y)) {
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

    mirror.print();

     let mut last_res = mirror.rocks.clone();

    for n in 0..1_000_000_000 {
        if n % 100 == 0 {
            println!("Round {n}");
        }

       mirror = north(mirror);
         mirror = west(mirror);
       mirror = south(mirror);
         mirror = east(mirror);

         if last_res.into_iter().cmp(mirror.rocks).is_eq() {
             break;
         }

         last_res = mirror.rocks.clone();

    }

    println!();
    mirror.print();

    0
}

fn east(mut mirror: Mirror) -> Mirror {
    (0..mirror.h).for_each(|y| {
        let mut target_pos = None;
        (0..mirror.w).rev().for_each(|x| {
            match mirror.rocks.get(&(x, y)) {
                None => match target_pos {
                    None => target_pos = Some((x, y)),
                    _ => {}
                },

                Some(Rock::Movable) => match target_pos {
                    None => {}
                    Some((x_target, y_target)) => {
                        mirror.rocks.insert((x_target, y_target), Rock::Movable);
                        mirror.rocks.remove(&(x, y));
                        target_pos = Some((x.max(x_target-1), y));
                    }
                },
                Some(Rock::Stable) => target_pos = None,
            }
        });
    });
    mirror
}

fn west(mut mirror: Mirror) -> Mirror {
    (0..mirror.h).for_each(|y| {
        let mut target_pos = None;
        (0..mirror.w).for_each(|x| {
            match mirror.rocks.get(&(x, y)) {
                None => match target_pos {
                    None => target_pos = Some((x, y)),
                    _ => {}
                },

                Some(Rock::Movable) => match target_pos {
                    None => {}
                    Some((x_target, y_target)) => {
                        mirror.rocks.insert((x_target, y_target), Rock::Movable);
                        mirror.rocks.remove(&(x, y));
                        target_pos = Some((x.min(x_target+1), y));
                    }
                },
                Some(Rock::Stable) => target_pos = None,
            }
        });
    });
    mirror
}

fn north(mut mirror: Mirror) -> Mirror {
    (0..mirror.w).for_each(|x| {
        let mut target_pos = None;
        (0..mirror.h).for_each(|y| {
            match mirror.rocks.get(&(x, y)) {
                None => match target_pos {
                    None => target_pos = Some((x, y)),
                    _ => {}
                },

                Some(Rock::Movable) => match target_pos {
                    None => {}
                    Some((x_target, y_target)) => {
                        mirror.rocks.insert((x_target, y_target), Rock::Movable);
                        mirror.rocks.remove(&(x, y));
                        target_pos = Some((x, y.min(y_target + 1)));
                    }
                },
                Some(Rock::Stable) => target_pos = None,
            }
        });
    });

    mirror
}

fn south(mut mirror: Mirror) -> Mirror {
    (0..mirror.w).for_each(|x| {
        let mut target_pos = None;
        (0..mirror.h).rev().for_each(|y| {
            match mirror.rocks.get(&(x, y)) {
                None => match target_pos {
                    None => target_pos = Some((x, y)),
                    _ => {}
                },

                Some(Rock::Movable) => match target_pos {
                    None => {}
                    Some((x_target, y_target)) => {
                        mirror.rocks.insert((x_target, y_target), Rock::Movable);
                        mirror.rocks.remove(&(x, y));
                        target_pos = Some((x, y.max(y_target - 1)));
                    }
                },
                Some(Rock::Stable) => target_pos = None,
            }
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
            w = w.max(x+1);
            h = h.max(y+1);
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
        assert_eq!(result, 64);
    }
}

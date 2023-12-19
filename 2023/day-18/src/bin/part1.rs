use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{self, alphanumeric1};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
struct Instruction<'a> {
    dir: Direction,
    length: u32,
    color: &'a str,
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    let (_, instr): (_, Vec<Instruction>) = parse_input(input).expect("Valid input");

    let (mut map, _, _) = instr.iter().fold(
        (HashMap::from([((0, 0), "".to_string())]), 0, 0),
        |(mut map, mut x, mut y), inst| {
            (0..inst.length).for_each(|_| {
                match inst.dir {
                    Direction::U => y -= 1,
                    Direction::D => y += 1,
                    Direction::L => x -= 1,
                    Direction::R => x += 1,
                };
                map.insert((x, y), inst.color.to_string());
            });

            (map, x, y)
        },
    );

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    map.keys().for_each(|(x, y)| {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    });

    let mut some_inner = None;
    'outer: for y in min_y..=max_y {
        let mut prepre = None;
        let mut pre = None;
        for x in min_x..=max_x {
            let act = map.get(&(x, y));

            if prepre.is_none() && pre.is_some() && act.is_none() {
                some_inner = Some((x, y));
                break 'outer;
            }

            prepre = pre;
            pre = act;
        }
    }

    if let Some(start) = some_inner {
        let mut to_fill: Vec<(i32, i32)> = vec![start];

        while let Some(xy) = to_fill.pop() {
            map.insert(xy, "p".to_string());

            for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nxy: (i32, i32) = (xy.0 + d.0, xy.1 + d.1);
                match map.get(&nxy) {
                    None => {
                        if !to_fill.contains(&nxy) {
                            to_fill.push(nxy)
                        }
                    }
                    Some(_) => (),
                }
            }
        }

        map.len() as u32
    } else {
        0
    }
}

fn _print_map(
    map: &HashMap<(i32, i32), String>,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
    start: (i32, i32),
) {
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match map.get(&(x, y)) {
                Some(_) => {
                    if start.0 == x && start.1 == y {
                        print!("S")
                    } else {
                        print!("#")
                    }
                }
                None => {
                    if start.0 == x && start.1 == y {
                        print!("N")
                    } else {
                        print!(".")
                    }
                }
            }
        }
        println!();
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(
        tag("\n"),
        tuple((
            alphanumeric1,
            tag(" "),
            complete::u32,
            tag(" ("),
            is_a("#0123456789abcdef"),
            tag(")"),
        )),
    )(input)?;

    let draw = instructions
        .iter()
        .map(|(d, _, l, _, c, _)| Instruction {
            color: c,
            dir: match *d {
                "D" => Direction::D,
                "U" => Direction::U,
                "L" => Direction::L,
                "R" => Direction::R,
                _ => panic!("Invalid input"),
            },
            length: *l,
        })
        .collect();

    Ok((&input, draw))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        let result = solve(
            "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        );
        assert_eq!(result, 62);
    }
}

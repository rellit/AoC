use colored::Colorize;
use std::{collections::HashSet, fmt::Display};

use rusttype::{Point, Vector};

type Direction = Vector<isize>;

#[derive(Debug, PartialEq)]
enum Dir {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Dir {
    fn value(&self) -> Direction {
        match *self {
            Dir::Up => Vector { x: 0, y: -1 },   //[0, -1],
            Dir::Right => Vector { x: 1, y: 0 }, //[1, 0],
            Dir::Down => Vector { x: 0, y: 1 },  //[0, 1],
            Dir::Left => Vector { x: -1, y: 0 }, //[-1, 0],
        }
    }

    fn opposite(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
        }
    }
}

impl From<isize> for Dir {
    fn from(value: isize) -> Self {
        match value {
            0 => Dir::Up,
            1 => Dir::Right,
            2 => Dir::Down,
            3 => Dir::Left,
            i => panic!("Invalid dir index {i}"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
enum Tile {
    NS,
    EW,
    NE,
    SE,
    SW,
    NW,
    Start,
    None,
}

impl Tile {
    fn can_go(&self, dir: &Dir) -> bool {
        match dir {
            Dir::Up => *self == Tile::NS || *self == Tile::NE || *self == Tile::NW,
            Dir::Right => *self == Tile::NE || *self == Tile::SE || *self == Tile::EW,
            Dir::Down => *self == Tile::NS || *self == Tile::SE || *self == Tile::SW,
            Dir::Left => *self == Tile::NW || *self == Tile::EW || *self == Tile::SW,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::NS => f.write_str("│"),
            Tile::EW => f.write_str("─"),
            Tile::NE => f.write_str("└"),
            Tile::SE => f.write_str("┌"),
            Tile::SW => f.write_str("┐"),
            Tile::NW => f.write_str("┘"),
            Tile::Start => f.write_str("S"),
            Tile::None => f.write_str("."),
        }
    }
}

struct Map {
    tile_array: Vec<Vec<Tile>>,
    loop_length: Option<usize>,
    loop_tiles: HashSet<Point<isize>>,
    outside_tiles: HashSet<Point<isize>>,
}

impl Map {
    fn get_at(&self, pos: &Point<isize>) -> Option<&Tile> {
        self.tile_array.get(pos.y as usize)?.get(pos.x as usize)
    }

    fn counts_as(&self, pos: Point<isize>) -> &Tile {
        let t: &Tile = self.get_at(&pos).expect("Valid Tile");
        if *t == Tile::Start {
            let mut p = 0;
            if let Some(possible) = self.get_at(&(pos + Dir::Up.value())) {
                if possible.can_go(&Dir::Down) {
                    p += 1
                }
            }
            if let Some(possible) = self.get_at(&(pos + Dir::Right.value())) {
                if possible.can_go(&Dir::Left) {
                    p += 2
                }
            }
            if let Some(possible) = self.get_at(&(pos + Dir::Down.value())) {
                if possible.can_go(&Dir::Up) {
                    p += 4
                }
            }
            if let Some(possible) = self.get_at(&(pos + Dir::Left.value())) {
                if possible.can_go(&Dir::Right) {
                    p += 8
                }
            }

            let tile = match p {
                3 => &Tile::NE,
                5 => &Tile::NS,
                9 => &Tile::NW,
                6 => &Tile::SE,
                10 => &Tile::EW,
                12 => &Tile::SW,
                _ => &Tile::None,
            };
            tile
        } else {
            t
        }
    }

    fn find_start(&self) -> Point<isize> {
        self.tile_array
            .iter()
            .enumerate()
            .find_map(|(y, tl)| {
                let row_idx = tl.iter().enumerate().find_map(|(x, t)| match t {
                    Tile::Start => Some(x),
                    _ => None,
                });
                row_idx.map(|x| Point {
                    y: y as isize,
                    x: x as isize,
                })
            })
            .expect("There is a start in Puzzle input")
    }

    fn get_next(&self, act: &Point<isize>, from: Option<&Point<isize>>) -> Point<isize> {
        let mut next = None;
        for d in 0..4 {
            let dir = Dir::try_from(d).expect("Valid Dir index");
            let pos = *act + dir.value();

            if let Some(from) = from {
                if pos.x == from.x && pos.y == from.y {
                    continue;
                }
            }

            match self.get_at(act).expect("Valid Tile") {
                Tile::Start => {
                    if let Some(possible) = self.get_at(&pos) {
                        if possible.can_go(&dir.opposite()) {
                            next = Some(pos);
                            break;
                        }
                    }
                }
                t => {
                    if t.can_go(&dir) {
                        next = Some(pos);
                        break;
                    }
                }
            }
        }

        next.expect("Valid loop")
    }

    fn find_loop(&mut self) {
        let start: Point<isize> = self.find_start();
        let first_step = self.get_next(&start, None);
        self.loop_length = Some(self.follow(&start, &first_step, 1));
    }

    fn follow(&mut self, from: &Point<isize>, act: &Point<isize>, steps: usize) -> usize {
        self.loop_tiles.insert(*act);
        if *self.get_at(act).expect("Valid tile") == Tile::Start {
            steps
        } else {
            let next = self.get_next(act, Some(from));
            self.follow(act, &next, steps + 1)
        }
    }

    fn mark_fields_outside(&mut self) {
        for (y, l) in self.tile_array.iter().enumerate() {
            let mut inside = false;
            let mut half_border = None;
            for (x, _) in l.iter().enumerate() {
                let p = Point {
                    x: x as isize,
                    y: y as isize,
                };

                if self.loop_tiles.contains(&p) {
                    match self.counts_as(p) {
                        Tile::NS => inside = !inside,
                        Tile::EW => (),
                        Tile::NE | Tile::NW => match half_border {
                            Some(h) => {
                                half_border = None;
                                if h != Dir::Up {
                                    inside = !inside;
                                }
                            }
                            None => half_border = Some(Dir::Up),
                        },
                        Tile::SE | Tile::SW => match half_border {
                            Some(h) => {
                                half_border = None;
                                if h != Dir::Down {
                                    inside = !inside;
                                }
                            }
                            None => half_border = Some(Dir::Down),
                        },
                        Tile::Start => (),
                        Tile::None => (),
                    }
                    continue;
                }

                if !self.outside_tiles.contains(&p) && !inside {
                    self.outside_tiles.insert(p);
                }
            }
        }
    }

    fn print(&self) {
        for (y, l) in self.tile_array.iter().enumerate() {
            for (x, t) in l.iter().enumerate() {
                let f = format!("{t}");
                if self.loop_tiles.contains(&Point {
                    x: x as isize,
                    y: y as isize,
                }) {
                    print!("{}", f.green())
                } else if self.outside_tiles.contains(&Point {
                    x: x as isize,
                    y: y as isize,
                }) {
                    print!("{}", f.red())
                } else {
                    print!("{}", f)
                }
            }
            println!()
        }
    }

    fn trapped_tiles(&self) -> usize {
        self.tile_array.len() * self.tile_array.first().unwrap().len()
            - (self.loop_length.expect("Loop was already found") + self.outside_tiles.len())
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let mut map: Map = parse_map(input);
    map.find_loop();
    map.mark_fields_outside();

    map.print();

    map.trapped_tiles()
}

fn parse_map(input: &str) -> Map {
    let tile_array = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    use Tile::*;
                    match c {
                        '|' => NS,
                        '-' => EW,
                        'L' => NE,
                        'F' => SE,
                        '7' => SW,
                        'J' => NW,
                        'S' => Start,
                        '.' => None,
                        c => panic!("Unknown Tile: {c}"),
                    }
                })
                .collect()
        })
        .collect();
    Map {
        tile_array,
        loop_length: None,
        loop_tiles: HashSet::new(),
        outside_tiles: HashSet::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        let result = solve(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, 1);
    }
    #[test]
    fn test_code2() {
        let result = solve(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );
        assert_eq!(result, 4);
    }
    #[test]
    fn test_code3() {
        let result = solve(
            "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........",
        );
        assert_eq!(result, 4);
    }
}

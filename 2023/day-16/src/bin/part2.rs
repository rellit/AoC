use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

use glam::IVec2;

const UP: IVec2 = IVec2::from_array([0, -1]);
const DOWN: IVec2 = IVec2::from_array([0, 1]);
const LEFT: IVec2 = IVec2::from_array([-1, 0]);
const RIGHT: IVec2 = IVec2::from_array([1, 0]);

#[derive(PartialEq, Clone)]
enum TileKind {
    Ver,
    Hor,
    Pos,
    Neg,
    Empty,
}
#[derive(PartialEq, Clone)]
struct Tile {
    kind: TileKind,
    energized_left: bool,
    energized_right: bool,
    energized_up: bool,
    energized_down: bool,
}

impl Tile {
    fn from(kind: TileKind) -> Self {
        Self {
            kind,
            energized_left: false,
            energized_right: false,
            energized_up: false,
            energized_down: false,
        }
    }

    fn energized(&self) -> bool {
        self.energized_left || self.energized_down || self.energized_right || self.energized_up
    }

    fn energized_from(&self, dir: IVec2) -> bool {
        match dir {
            UP => self.energized_up,
            DOWN => self.energized_down,
            LEFT => self.energized_left,
            RIGHT => self.energized_right,
            _ => false,
        }
    }

    fn energize_from(&mut self, dir: IVec2) {
        match dir {
            UP => self.energized_up = true,
            DOWN => self.energized_down = true,
            LEFT => self.energized_left = true,
            RIGHT => self.energized_right = true,
            _ => (),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.energized() {
            write!(f, "#")?;
        } else {
            match self.kind {
                TileKind::Ver => write!(f, "|")?,
                TileKind::Hor => write!(f, "-")?,
                TileKind::Pos => write!(f, "/")?,
                TileKind::Neg => write!(f, "\\")?,
                TileKind::Empty => write!(f, ".")?,
            };
        }
        Ok(())
    }
}

type MapInfo = (usize, usize, HashMap<IVec2, Tile>);
type Map = HashMap<IVec2, Tile>;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let (x_max, y_max, map): MapInfo = parse_input(input);

    let mut max: usize = 0;

    for y in 0..=y_max {
        let mut m = map.clone();
        energize(&mut m, IVec2::from_array([0, y as i32]), RIGHT);
        max = max.max(m.values().filter(|t| t.energized()).count());
        m = map.clone();
        energize(&mut m, IVec2::from_array([x_max as i32, y as i32]), LEFT);
        max = max.max(m.values().filter(|t| t.energized()).count())
    }
    for x in 0..=x_max {
        let mut m = map.clone();
        energize(&mut m, IVec2::from_array([x as i32, 0]), DOWN);
        max = max.max(m.values().filter(|t| t.energized()).count());
        m = map.clone();
        energize(&mut m, IVec2::from_array([x as i32, y_max as i32]), UP);
        max = max.max(m.values().filter(|t| t.energized()).count())
    }

    max
}

fn _print_map(x_max: usize, y_max: usize, map: &Map) {
    for y in 0..=y_max {
        for x in 0..=x_max {
            print!(
                "{t}",
                t = map
                    .get(&IVec2 {
                        x: x as i32,
                        y: y as i32
                    })
                    .unwrap()
            )
        }
        println!();
    }
}

fn energize(map: &mut Map, mut curr_pos: IVec2, direction: IVec2) {
    while let Some(tile) = map.get_mut(&curr_pos) {
        if tile.energized_from(direction) {
            return;
        }

        tile.energize_from(direction);
        if tile.kind == TileKind::Empty {
            curr_pos += direction;
        } else {
            break;
        }
    }

    if let Some(tile) = map.get_mut(&curr_pos) {
        match tile.kind {
            TileKind::Empty => {}
            TileKind::Ver => match direction {
                LEFT | RIGHT => {
                    energize(map, curr_pos + UP, UP);
                    energize(map, curr_pos + DOWN, DOWN);
                }
                _ => energize(map, curr_pos + direction, direction),
            },
            TileKind::Hor => match direction {
                UP | DOWN => {
                    energize(map, curr_pos + LEFT, LEFT);
                    energize(map, curr_pos + RIGHT, RIGHT);
                }
                _ => energize(map, curr_pos + direction, direction),
            },
            TileKind::Pos => match direction {
                LEFT => energize(map, curr_pos + DOWN, DOWN),
                RIGHT => energize(map, curr_pos + UP, UP),
                UP => energize(map, curr_pos + RIGHT, RIGHT),
                DOWN => energize(map, curr_pos + LEFT, LEFT),

                _ => {}
            },
            TileKind::Neg => match direction {
                LEFT => energize(map, curr_pos + UP, UP),
                RIGHT => energize(map, curr_pos + DOWN, DOWN),
                UP => energize(map, curr_pos + LEFT, LEFT),
                DOWN => energize(map, curr_pos + RIGHT, RIGHT),

                _ => {}
            },
        }
    }
}

fn parse_input(input: &str) -> MapInfo {
    let mut x_max = 0;
    let mut y_max = 0;
    let map: Map = input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                x_max = x_max.max(x);
                y_max = y_max.max(y);
                if let Some(kind) = match c {
                    '/' => Some(TileKind::Pos),
                    '\\' => Some(TileKind::Neg),
                    '-' => Some(TileKind::Hor),
                    '|' => Some(TileKind::Ver),
                    '.' => Some(TileKind::Empty),
                    _ => None,
                } {
                    map.insert(IVec2::from_array([x as i32, y as i32]), Tile::from(kind));
                }
            });

            map
        });
    (x_max, y_max, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        let result = solve(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        assert_eq!(result, 51);
    }
}

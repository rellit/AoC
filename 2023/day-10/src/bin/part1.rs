use rusttype::{Point, Vector};

type Direction = Vector<isize>;

#[derive(Debug)]
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

#[derive(PartialEq, Debug)]
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

struct Map {
    tile_array: Vec<Vec<Tile>>,
}

impl Map {
    fn _width(&self) -> usize {
        self.tile_array.first().unwrap().len()
    }
    fn _height(&self) -> usize {
        self.tile_array.len()
    }
    fn get_at(&self, pos: &Point<isize>) -> Option<&Tile> {
        self.tile_array.get(pos.y as usize)?.get(pos.x as usize)
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

    fn loop_length(&self, start: &Point<isize>) -> usize {
        let first_step = self.get_next(start, None);
        self.follow(start, &first_step, 1)
    }

    fn follow(&self, from: &Point<isize>, act: &Point<isize>, steps: usize) -> usize {
        if *self.get_at(act).expect("Valid tile") == Tile::Start {
            steps
        } else {
            let next = self.get_next(act, Some(from));
            self.follow(act, &next, steps + 1)
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let map: Map = parse_map(input);

    let start = map.find_start();

    let ll: usize = map.loop_length(&start);
    ll / 2
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
    Map { tile_array }
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
        assert_eq!(result, 4);
    }
}

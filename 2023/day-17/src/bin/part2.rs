use std::cmp::Reverse;
use std::collections::HashSet;

use glam::IVec2;
use priority_queue::PriorityQueue;

type Map = Vec<Vec<u8>>;

const UP: IVec2 = IVec2::from_array([0, -1]);
const DOWN: IVec2 = IVec2::from_array([0, 1]);
const LEFT: IVec2 = IVec2::from_array([-1, 0]);
const RIGHT: IVec2 = IVec2::from_array([1, 0]);
const DIRECTIONS: [IVec2; 4] = [UP, RIGHT, DOWN, LEFT];

#[derive(Debug)]
struct MapInfo {
    map: Map,
    width: usize,
    height: usize,
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    let map: MapInfo = parse_input(input);

    find_min_hl(&map).expect("Valid path exists")
}

fn find_min_hl(map: &MapInfo) -> Option<u32> {
    type Entry = (u32, u32, i32, i32, u8);

    let mut seen: HashSet<Entry> = HashSet::new();
    let mut pq = PriorityQueue::new();
    pq.push((0, 0, 0, 0, 0, 0), Reverse(0u32));

    while let Some(((x, y, dx, dy, n, hl), _)) = pq.pop() {
        if x == map.width as u32 - 1 && y == map.height as u32 - 1 && n >= 4 {
            println!("{pq:?}");
            return Some(hl);
        }

        if seen.contains(&(x, y, dx, dy, n)) {
            continue;
        }

        seen.insert((x, y, dx, dy, n));

        if n < 10 && (dx, dy) != (0, 0) {
            let nx: i32 = x as i32 + dx;
            let ny: i32 = y as i32 + dy;

            if 0 <= nx && nx < map.width as i32 && 0 <= ny && ny < map.height as i32 {
                //Keep going in this dir
                let nhl = *(map.map.get(ny as usize).unwrap().get(nx as usize).unwrap()) as u32;
                pq.push(
                    (nx as u32, ny as u32, dx, dy, n + 1, hl + nhl),
                    Reverse(hl + nhl),
                );
            }
        }

        if n >= 4 || (dx, dy) == (0, 0) {
            for dir in DIRECTIONS {
                if (dir.x, dir.y) != (dx, dy) && (dir.x, dir.y) != (-dx, -dy) {
                    let nx: i32 = x as i32 + dir.x;
                    let ny: i32 = y as i32 + dir.y;

                    if 0 <= nx && nx < map.width as i32 && 0 <= ny && ny < map.height as i32 {
                        //Keep going in this dir
                        let nhl =
                            *(map.map.get(ny as usize).unwrap().get(nx as usize).unwrap()) as u32;
                        pq.push(
                            (nx as u32, ny as u32, dir.x, dir.y, 1, hl + nhl),
                            Reverse(hl + nhl),
                        );
                    }
                }
            }
        }
    }

    None
}

fn parse_input(input: &str) -> MapInfo {
    let map: Map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    c.to_string()
                        .parse::<u8>()
                        .expect("Only valid nums in input")
                })
                .collect()
        })
        .collect();
    let width = map.first().unwrap().len();
    let height = map.len();
    MapInfo { map, width, height }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        let result = solve(
            "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        );
        assert_eq!(result, 94);
    }
    #[test]
    fn test_code_2() {
        let result = solve(
            "111111111111
999999999991
999999999991
999999999991
999999999991",
        );
        assert_eq!(result, 71);
    }
}

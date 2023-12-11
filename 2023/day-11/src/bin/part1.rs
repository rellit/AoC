#[derive(PartialEq, Clone, Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let map: Vec<Galaxy> = parse_map(input);

    map.iter()
        .enumerate()
        .map(|(idx, e)| {
            map.iter()
                .take(idx + 1)
                .map(|g| {
                    (g.x as isize - e.x as isize).unsigned_abs()
                        + (g.y as isize - e.y as isize).unsigned_abs()
                })
                .sum::<usize>()
        })
        .sum()
}

fn parse_map(input: &str) -> Vec<Galaxy> {
    let mut extra_lines = 0;
    let mut map: Vec<Galaxy> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            if l.chars().all(|c| c == '.') {
                extra_lines += 1;
                vec![]
            } else {
                l.chars()
                    .enumerate()
                    .filter_map(|(x, c)| match c {
                        '#' => Some(Galaxy {
                            x,
                            y: y + extra_lines,
                        }),
                        _ => None,
                    })
                    .collect()
            }
        })
        .collect();

    let max_x = map
        .iter()
        .max_by_key(|e| e.x)
        .expect("Input contains Galaxies")
        .x;

    (0..max_x).rev().for_each(|x| {
        if map.iter().all(|e| e.x != x) {
            map.iter_mut().filter(|e| e.x > x).for_each(|e| e.x += 1)
        }
    });
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        let result = solve(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, 374);
    }
}

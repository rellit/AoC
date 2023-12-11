#[derive(PartialEq, Clone, Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input, 1_000_000);
    dbg!(output);
}

fn solve(input: &str, growth: usize) -> usize {
    let map: Vec<Galaxy> = parse_map(input, growth);

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

fn parse_map(input: &str, growth: usize) -> Vec<Galaxy> {
    let mut extra_lines = 0;
    let mut map: Vec<Galaxy> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            if l.chars().all(|c| c == '.') {
                extra_lines += growth - 1;
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

    let max_x = map.iter().fold(0, |acc, e| acc.max(e.x));

    (0..max_x).rev().for_each(|x| {
        if map.iter().all(|e| e.x != x) {
            map.iter_mut()
                .filter(|e| e.x > x)
                .for_each(|e| e.x += growth - 1)
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
            10,
        );
        assert_eq!(result, 1030);
    }

    #[test]
    fn test_code_2() {
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
            100,
        );
        assert_eq!(result, 8410);
    }
}

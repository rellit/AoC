use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let mut lines = parse_input(input);

    let mut last_res = lines.clone();

    for n in 0..1_000_000_000 {
        if n % 100 == 0 {
            println!("Round {n}");
        }

        lines = north(lines);
        lines = west(lines);
        lines = south(lines);
        lines = east(lines);

        if last_res.cmp(&lines).is_eq() {
            break;
        }

        last_res = lines.clone();
    }

    lines
        .iter()
        .rev()
        .enumerate()
        .map(|(i, l)| {
            println!("{l}  {i:3}", i = i + 1);

            l.chars().filter(|c| *c == 'O').count() * (i + 1)
        })
        .sum()
}

fn east(mut lines: Vec<String>) -> Vec<String> {
    lines.par_iter_mut().for_each(|l| {
        while l.contains("O..") {
            *l = l.replace("O..", "..O");
        }
        while l.contains("O.") {
            *l = l.replace("O.", ".O");
        }
    });
    lines
}

fn west(mut lines: Vec<String>) -> Vec<String> {
    lines.par_iter_mut().for_each(|l| {
        while l.contains("..O") {
            *l = l.replace("..O", "O..");
        }
        while l.contains(".O") {
            *l = l.replace(".O", "O.");
        }
    });
    lines
}

fn north(mut lines: Vec<String>) -> Vec<String> {
    loop {
        let mut shifted = false;
        for n in 0..lines.len() - 1 {
            let mut new_upper = "".to_string();
            let mut new_curr = "".to_string();
            lines
                .get(n)
                .unwrap()
                .chars()
                .zip(lines.get(n + 1).unwrap().chars())
                .for_each(|(u, c)| {
                    if u == '.' && c == 'O' {
                        shifted = true;
                        new_upper.push('O');
                        new_curr.push('.');
                    } else {
                        new_upper.push(u);
                        new_curr.push(c);
                    }
                });
            *lines.get_mut(n).unwrap() = new_upper;
            *lines.get_mut(n + 1).unwrap() = new_curr;
        }
        if !shifted {
            break;
        }
    }
    lines
}

fn south(mut lines: Vec<String>) -> Vec<String> {
    loop {
        let mut shifted = false;
        for n in 0..lines.len() - 1 {
            let mut new_upper = "".to_string();
            let mut new_curr = "".to_string();
            lines
                .get(n)
                .unwrap()
                .chars()
                .zip(lines.get(n + 1).unwrap().chars())
                .for_each(|(u, c)| {
                    if u == 'O' && c == '.' {
                        shifted = true;
                        new_upper.push('.');
                        new_curr.push('O');
                    } else {
                        new_upper.push(u);
                        new_curr.push(c);
                    }
                });
            *lines.get_mut(n).unwrap() = new_upper;
            *lines.get_mut(n + 1).unwrap() = new_curr;
        }
        if !shifted {
            break;
        }
    }
    lines
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
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

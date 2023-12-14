fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let mut lines = parse_input(input);

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
        .iter()
        .rev()
        .enumerate()
        .map(|(i, l)| l.chars().filter(|c| *c == 'O').count() * (i + 1))
        .sum()
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
        assert_eq!(result, 136);
    }
}

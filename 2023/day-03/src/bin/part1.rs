#[derive(Debug)]
struct Num {
    line: usize,
    start: usize,
    len: usize,
    num: u32,
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    let mut numbers: Vec<Num> = Vec::new();
    let mut line_vec: Vec<&str> = Vec::new();
    input.lines().enumerate().for_each(|(line_nr, line)| {
        line_vec.push(line);
        let mut start: Option<usize> = None;
        let mut num: u32 = 0;
        let mut char_iter = line.chars().enumerate().peekable();

        while let Some((idx, char)) = char_iter.next() {
            if char.is_ascii_digit() {
                if start.is_none() {
                    start = Some(idx)
                }
                num = num * 10 + char.to_digit(10).unwrap();

                if (char_iter.peek().is_none() || !char_iter.peek().unwrap().1.is_ascii_digit())
                    && start.is_some()
                {
                    numbers.push(Num {
                        line: (line_nr),
                        start: (start.expect("Geprüft")),
                        len: idx + 1 - start.expect("Geprüft"),
                        num,
                    });
                    num = 0;
                    start = None;
                }
            }
        }
    });

    let valid_numbers = numbers.iter().filter_map(|num| {
        if is_symbol(get(
            &line_vec,
            num.line as isize,
            num.start as isize + num.len as isize,
        )) {
            return Some(num.num);
        }

        for idx in num.start as isize - 1..num.start as isize + num.len as isize + 1 {
            if is_symbol(get(&line_vec, num.line as isize - 1, idx))
                || is_symbol(get(&line_vec, num.line as isize + 1, idx))
            {
                return Some(num.num);
            }
        }

        None
    });

    valid_numbers.sum()
}

fn get(line_vec: &[&str], line: isize, idx: isize) -> Option<char> {
    if line < 0 {
        return None;
    }
    if idx < 0 {
        return None;
    }
    line_vec.get(line as usize)?.chars().nth(idx as usize)
}

fn is_symbol(char: Option<char>) -> bool {
    !(char.is_none() || char.unwrap().is_ascii_digit() || char.unwrap() == '.')
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = solve(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
",
        );
        assert_eq!(result, 4361);
    }
}

#[derive(Debug)]
struct Num {
    line: usize,
    start: usize,
    len: usize,
    num: u32,
}

fn main() {
    let input = include_str!("../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut numbers: Vec<Num> = Vec::new();
    let mut line_vec: Vec<&str> = Vec::new();
    input.lines().enumerate().for_each(|(line_nr, line)| {
        line_vec.push(line);
        let mut start: Option<usize> = None;
        let mut num: u32 = 0;
        let mut char_iter = line.chars().enumerate().peekable();

        while let Some((idx, char)) = char_iter.next() {
            if char.is_digit(10) {
                if start.is_none() {
                    start = Some(idx)
                }
                num = num * 10 + char.to_digit(10).unwrap();

                if char_iter.peek().is_none() || !char_iter.peek().unwrap().1.is_digit(10) {
                    if start.is_some() {
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
        }
    });

    let valid_numbers = numbers.iter().filter_map(|num| {
        if is_symbol(get(&line_vec, num.line as isize, num.start as isize - 1)) {
            return Some(num.num);
        } else if is_symbol(get(
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

        return None;
    });

    valid_numbers.sum()
}

fn get(line_vec: &Vec<&str>, line: isize, idx: isize) -> Option<char> {
    if line < 0 {
        return None;
    }
    if idx < 0 {
        return None;
    }
    line_vec.get(line as usize)?.chars().nth(idx as usize)
}

fn is_symbol(char: Option<char>) -> bool {
    if char.is_none() {
        false
    } else if char.expect("Proved in first compare").is_digit(10) {
        false
    } else if char.expect("msg") == '.' {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = part1(
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

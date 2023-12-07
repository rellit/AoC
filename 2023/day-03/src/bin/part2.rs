#[derive(Debug)]
struct Num {
    line: isize,
    start: isize,
    len: isize,
    num: u32,
}

impl Num {
    fn is_adjacent(&self, gear: &Gear) -> bool {
        if gear.line == self.line
            && (gear.idx == self.start - 1 || gear.idx == self.start + self.len)
        {
            return true;
        }

        if (gear.line + 1 == self.line || gear.line - 1 == self.line)
            && (gear.idx >= self.start - 1 && gear.idx <= self.start + self.len)
        {
            return true;
        }

        false
    }
}

struct Gear {
    line: isize,
    idx: isize,
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    let mut numbers: Vec<Num> = Vec::new();
    let mut gears: Vec<Gear> = Vec::new();
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
                        line: (line_nr as isize),
                        start: (start.expect("Geprüft") as isize),
                        len: idx as isize + 1 - start.expect("Geprüft") as isize,
                        num,
                    });
                    num = 0;
                    start = None;
                }
            }
            if char == '*' {
                gears.push(Gear {
                    line: line_nr as isize,
                    idx: idx as isize,
                })
            }
        }
    });

    let gear_ratios: Vec<u32> = gears
        .iter()
        .filter_map(|gear| {
            let pair: Vec<u32> = numbers
                .iter()
                .filter_map(|num| {
                    if num.is_adjacent(gear) {
                        Some(num.num)
                    } else {
                        None
                    }
                })
                .collect();

            if pair.len() == 2 {
                Some(pair.iter().product())
            } else {
                None
            }
        })
        .collect();

    gear_ratios.iter().sum()
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
.664.598..",
        );
        assert_eq!(result, 467835);
    }
}

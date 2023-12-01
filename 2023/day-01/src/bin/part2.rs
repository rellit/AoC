fn main() {
    let input = include_str!("../input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let mut digits: Vec<u32> = Vec::new();

    for line in input.lines().into_iter() {
        let line_digits: Vec<u32> = parse(line);
        digits.push(line_digits.first().unwrap() * 10);
        digits.push(line_digits.last().unwrap() * 1);
    }
    digits.into_iter().reduce(|acc, d| acc + d).unwrap()
}

fn parse(line: &str) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();

    let line = line.trim();

    let mut it = line.chars().enumerate();
    while let Some((idx, char)) = it.next() {
        if line.get(idx..).unwrap().starts_with("one") {
            digits.push(1);
            continue;
        }
        if line.get(idx..).unwrap().starts_with("two") {
            digits.push(2);
            continue;
        }
        if line.get(idx..).unwrap().starts_with("three") {
            digits.push(3);
            continue;
        }
        if line.get(idx..).unwrap().starts_with("four") {
            digits.push(4);
            continue;
        }
        if line.get(idx..).unwrap().starts_with("five") {
            digits.push(5);
            continue;
        }
        if line.get(idx..).unwrap().starts_with("six") {
            digits.push(6);
            continue;
        }
        if line.get(idx..).unwrap().starts_with("seven") {
            digits.push(7);
            continue;
        }
        if line.get(idx..).unwrap().starts_with("eight") {
            digits.push(8);
            continue;
        }
        if line.get(idx..).unwrap().starts_with("nine") {
            digits.push(9);
            continue;
        }
        if char.is_digit(10) {
            digits.push(char.to_digit(10).unwrap());
            continue;
        }
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = part2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}

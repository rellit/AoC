fn main() {
    let input = include_str!("../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut digits: Vec<u32> = Vec::new();

    for line in input.lines().into_iter() {
        let mut line_digits: Vec<u32> = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                line_digits.push(c.to_digit(10).unwrap())                
            }
        }
        digits.push(line_digits.first().unwrap() * 10);
        digits.push(line_digits.last().unwrap() * 1);
    }

    return digits.into_iter().reduce(|acc, d| acc + d).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = part1(
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        );
        assert_eq!(result, 142);
    }
}

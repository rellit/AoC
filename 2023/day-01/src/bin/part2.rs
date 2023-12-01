fn main() {
    let input = include_str!("../input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut it = line.chars().enumerate().filter_map(|(i, c)| {
                if let Some(d) = c.to_digit(10) {
                    Some(d)
                } else if (&line[i..]).starts_with("one") {
                    Some(1)
                } else if (&line[i..]).starts_with("two") {
                    Some(2)
                } else if (&line[i..]).starts_with("three") {
                    Some(3)
                } else if (&line[i..]).starts_with("four") {
                    Some(4)
                } else if (&line[i..]).starts_with("five") {
                    Some(5)
                } else if (&line[i..]).starts_with("six") {
                    Some(6)
                } else if (&line[i..]).starts_with("seven") {
                    Some(7)
                } else if (&line[i..]).starts_with("eight") {
                    Some(8)
                } else if (&line[i..]).starts_with("nine") {
                    Some(9)
                } else {
                    None
                }
            });
            let first = it.next().expect("There sould be at leas 1 number");
            match it.last() {
                Some(num) => first * 10 + num,
                None => {
                    first * 10 + first
                }
            }
        })
        .sum::<u32>()
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

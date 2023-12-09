use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .tuples()
        .flat_map(|(f, m, l)| {
            f.chars()
                .filter(|c| m.contains(*c) && l.contains(*c))
                .collect::<HashSet<char>>()
        })
        .map(|c| {
            if c.is_lowercase() {
                c as u32 - 96
            } else {
                c as u32 - 38
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = solve(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(result, 70);
    }
}

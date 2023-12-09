use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let left = &l[0..l.len() / 2];
            let right = &l[l.len() / 2..l.len()];

            (left, right)
        })
        .filter_map(|(left, right)| {
            let f = left
                .chars()
                .filter(|c| right.contains(*c))
                .collect::<Vec<char>>();
            if !f.is_empty() {
                Some(f)
            } else {
                None
            }
        })
        .flat_map(|c| {
            c.iter()
                .collect::<HashSet<&char>>()
                .iter()
                .map(|c| {
                    if c.is_lowercase() {
                        **c as u32 - 96
                    } else {
                        **c as u32 - 38
                    }
                })
                .collect::<Vec<u32>>()
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
        assert_eq!(result, 157);
    }
}

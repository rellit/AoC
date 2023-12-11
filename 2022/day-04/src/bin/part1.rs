use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(',');
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|(left, right)| {
            let mut split = left.split('-');
            let left = RangeInclusive::new(
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            );
            let mut split = right.split('-');
            let right = RangeInclusive::new(
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            );
            (left, right)
        })
        .filter(|(left, right)| {
            (left.contains(right.start()) && left.contains(right.end()))
                || (right.contains(left.start()) && right.contains(left.end()))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = solve(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        );
        assert_eq!(result, 2);
    }
}

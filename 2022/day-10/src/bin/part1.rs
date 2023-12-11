use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> i32 {
    let mut x: i32 = 1;
    let mut cycle = 0;
    let mut sum = 0;

    input.lines().for_each(|l| {
        let mut a = 0;
        match l {
            "noop" => (),
            s if s.starts_with("addx") => {
                cycle += 1;
                if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                    sum += cycle * x;
                }
                let s: Vec<&str> = s.split_whitespace().collect();
                a = (*s.get(1).expect("Has arg"))
                    .parse::<i32>()
                    .expect("Valid arg");
            }
            _ => (),
        }

        cycle += 1;
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            sum += cycle * x;
        }
        x += a;
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = solve(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        );
        assert_eq!(result, 13140);
    }
}

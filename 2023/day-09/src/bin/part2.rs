use nom::{bytes::complete::tag, character::complete, multi::separated_list1, IResult};

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> i32 {
    let histories: Vec<Vec<i32>> = parse_hitories(input).unwrap().1;

    histories
        .iter()
        .map(|history| {
            let mut diff_vecs: Vec<Vec<i32>> = vec![history.clone()];
            loop {
                diff_vecs.push(get_next_line(diff_vecs.last().unwrap()));
                if diff_vecs.last().unwrap().iter().all(|u| *u == 0) {
                    break;
                }
            }

            diff_vecs
                .iter()
                .rev()
                .skip(1)
                .fold(0, |a, v| v.first().unwrap() - a)
        })
        .sum()
}

fn get_next_line(line: &[i32]) -> Vec<i32> {
    line.windows(2)
        .map(|a| a.last().unwrap() - a.first().unwrap())
        .collect()
}

fn parse_hitories(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, hitories) =
        separated_list1(tag("\n"), separated_list1(tag(" "), complete::i32))(input)?;

    Ok((&input, hitories))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        let result = solve(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test_1() {
        let result = solve("0 3 6 9 12 15");
        assert_eq!(result, -3);
    }

    #[test]
    fn test_2() {
        let result = solve("1 3 6 10 15 21");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let result = solve("10 13 16 21 30 45");
        assert_eq!(result, 5);
    }
}

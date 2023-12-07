use nom::{
    bytes::complete::tag,
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u64 {
    let (time, dist) = racing_table(input).unwrap().1;

    ways_to_beat(&dist, &time)
}

fn racing_table(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, time) = separated_list1(space1, complete::digit1)(input)?;
    let (input, _) = tag("\nDistance:")(input)?;
    let (input, _) = space1(input)?;
    let (input, dist) = separated_list1(space1, complete::digit1)(input)?;

    let time: u64 = time
        .iter()
        .map(|s| s.to_string())
        .collect::<String>()
        .parse()
        .unwrap();
    let dist: u64 = dist
        .iter()
        .map(|s| s.to_string())
        .collect::<String>()
        .parse()
        .unwrap();

    Ok((&input, (time, dist)))
}

fn ways_to_beat(dist: &u64, time: &u64) -> u64 {
    let mut min = 0;

    for t in 0..*time {
        if t * (time - t) > *dist {
            min = t;
            break;
        }
    }

    time - 2 * min + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = solve(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, 71503);
    }
}

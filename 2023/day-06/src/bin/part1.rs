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

fn solve(input: &str) -> u32 {
    let (time, dist) = racing_table(input).unwrap().1;

    let ways_to_beat: Vec<u32> = time
        .iter()
        .zip(dist.iter())
        .map(|(t, d)| ways_to_beat(d, t))
        .collect();

    ways_to_beat.iter().product()
}

fn racing_table(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, time) = separated_list1(space1, complete::u32)(input)?;
    let (input, _) = tag("\nDistance:")(input)?;
    let (input, _) = space1(input)?;
    let (input, dist) = separated_list1(space1, complete::u32)(input)?;

    Ok((&input, (time, dist)))
}

fn ways_to_beat(dist: &u32, time: &u32) -> u32 {
    let mut min = 0;
    let mut max = 0;

    for t in 0..*time {
        if beats(dist, t, time) {
            min = t;
            break;
        }
    }

    for t in (0..*time).into_iter().rev() {
        if beats(dist, t, time) {
            max = t;
            break;
        }
    }

    max + 1 - min
}

fn beats(dist: &u32, hold: u32, duration: &u32) -> bool {
    hold * (duration - hold) > *dist
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
        assert_eq!(result, 288);
    }
}

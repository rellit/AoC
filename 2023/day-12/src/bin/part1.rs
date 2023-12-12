use nom::{
    bytes::complete::{tag, take_until},
    character::complete,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(PartialEq, Clone, Debug)]
struct SpringMapLine<'a> {
    map: &'a str,
    groups: Vec<u32>,
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let (_, map) = parse_map(input).expect("Valid input");

    map.iter()
        .map(|line| {
            let placeholders = line.map.chars().filter(|c| *c == '?').count();
            let hashs = line.map.chars().filter(|c| *c == '#').count();
            let group_sum = line.groups.iter().sum::<u32>() as usize;
            let max_options = 2u32.pow(placeholders as u32);

            (0..max_options)
                .map(|p| format!("{p:0>width$b}", width = placeholders))
                .filter(|mask| mask.chars().filter(|c| *c == '1').count() + hashs == group_sum)
                .filter(|mask| {
                    let res = mask.chars().fold(line.map.to_string(), |acc, c| {
                        acc.replacen(
                            '?',
                            match c {
                                '1' => "#",
                                '0' => ".",
                                _ => panic!("Invalid bitmask"),
                            },
                            1,
                        )
                    });

                    valid_pattern(&res, &line.groups)
                })
                .count()
        })
        .sum()
}

fn valid_pattern(map: &str, groups: &[u32]) -> bool {
    let mut r = Vec::new();
    let mut last = None;
    let mut c: u32 = 0;
    map.chars().for_each(|chr| {
        match chr {
            '.' => {
                if let Some('#') = last {
                    r.push(c);
                    c = 0;
                }
            }
            '#' => c += 1,
            _ => (),
        }

        last = Some(chr);
    });
    if map.ends_with('#') {
        r.push(c);
    }
    let mtch = r.len() == groups.len() && r.iter().zip(groups.iter()).all(|(r, l)| *r == *l);

    mtch
}

fn parse_map(input: &str) -> IResult<&str, Vec<SpringMapLine>> {
    let (input, lines) = separated_list1(tag("\n"), parse_line)(input)?;

    Ok((&input, lines))
}

fn parse_line(input: &str) -> IResult<&str, SpringMapLine> {
    let (input, (map, groups)) = separated_pair(
        take_until(" "),
        tag(" "),
        separated_list1(tag(","), complete::u32),
    )(input)?;

    Ok((&input, SpringMapLine { map, groups }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        let result = solve(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        assert_eq!(result, 21);
    }

    #[test]
    fn test_code_1() {
        let result = solve("???.### 1,1,3");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_code_2() {
        let result = solve(".??..??...?##. 1,1,3");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_code_3() {
        let result = solve("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_code_4() {
        let result = solve("????.#...#... 4,1,1");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_code_5() {
        let result = solve("????.######..#####. 1,6,5");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_code_6() {
        let result = solve("?###???????? 3,2,1");
        assert_eq!(result, 10);
    }
}

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

fn count(map: &str, groups: &[u32]) -> u32 {
    println!("Call count with {map}, {groups:?}");

    if map.is_empty() {
        if groups.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }
    if groups.is_empty() {
        if map.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    let mut res = 0;

    if map.starts_with('.') || map.starts_with('?') {
        res += count(map.get(1..).unwrap(), groups);
    }

    if map.starts_with('#') || map.starts_with('?') {
        let fg = *groups.first().unwrap() as usize;
        if fg <= map.len()
            && !(map.get(..fg).unwrap().contains('.'))
            && (fg == map.len() || map.get(fg..fg + 1).unwrap() != "#")
            && map.get(fg + 1..).is_some()
        {
            res += count(map.get(fg + 1..).unwrap(), groups.get(1..).unwrap())
        } 
    }

    res
}

fn solve(input: &str) -> u32 {
    let (_, map) = parse_map(input).expect("Valid input");

    map.iter().map(|l| count(l.map, &l.groups)).sum()
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

use nom::{
    bytes::complete::{tag, take_until, take_until1},
    multi::separated_list1,
    IResult,
};

#[derive(PartialEq, Clone, Debug)]
struct SpringMapLine<'a> {
    map: &'a str,
    groups: Vec<usize>,
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let (_, map) = parse_input(input).expect("Valid input");

    map.iter()
        .map(|picture| {
            if let Some(h) = maps_horizontal_at(picture) {
                h * 100
            } else {
                maps_vertical_at(picture).expect("Should reflect in either dir")
            }
        })
        .sum()
}

fn maps_horizontal_at(picture: &[&str]) -> Option<usize> {
    for n in 0..picture.len() - 1 {
        //n means potential Split after row n
        // println!("{n}");
        let mut matches = true;
        for cmp in (0..n + 1).rev() {
            // println!(
            //     "Schould compare {l} with {r}",
            //     l = cmp,
            //     r = (2 * n + 1) - cmp
            // );

            let l = picture.get(cmp);
            let r = picture.get((2 * n + 1) - cmp);
            if l.is_none() || r.is_none() {
                break;
            } else if l.unwrap() != r.unwrap() {
                matches = false;
                break;
            }
        }
        if matches {
            return Some(n + 1);
        }

        //0 -> 0 and  1
        //     n and n+1

        //1 -> 0 and  3 ,  1  and  2
        //     n and n+3, n+1 and n+2

        //2 -> 0 and  5 ,  1  and  4 ,  2  and  3
        //     n and n+5, n+1 and n+4, n+1 and n+2
    }

    None
}

fn maps_vertical_at(picture: &[&str]) -> Option<usize> {
    for n in 0..picture.first().unwrap().len() - 1 {
        //n means potential Split after row n
        // println!("{n}");
        let mut matches = true;
        for cmp in (0..n + 1).rev() {
            // println!(
            //     "Schould compare {l} with {r}",
            //     l = cmp,
            //     r = (2 * n + 1) - cmp
            // );

            if !picture
                .iter()
                .map(|line| {
                    let l = line.char_indices().nth(cmp);
                    let r = line.char_indices().nth((2 * n + 1) - cmp);
                    (l, r)
                })
                .all(|(l, r)| {
                    // println!("Compare {l:?} - {r:?}");
                    l.is_none() || r.is_none() || l.unwrap().1 == r.unwrap().1
                })
            {
                matches = false;
                break;
            }
        }
        if matches {
            return Some(n + 1);
        }

        //0 -> 0 and  1
        //     n and n+1

        //1 -> 0 and  3 ,  1  and  2
        //     n and n+3, n+1 and n+2

        //2 -> 0 and  5 ,  1  and  4 ,  2  and  3
        //     n and n+5, n+1 and n+4, n+1 and n+2
    }

    None
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    let (input, lines) =
        separated_list1(tag("\n\n"), separated_list1(tag("\n"), take_until1("\n")))(input)?;

    Ok((&input, lines))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        let result = solve(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, 405);
    }

    #[test]
    fn test_code_1() {
        let result = solve(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );
        assert_eq!(result, 5);
    }

    #[test]
    fn test_code_2() {
        let result = solve(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, 400);
    }
}

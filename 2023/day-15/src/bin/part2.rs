use core::panic;
use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::{
    character::complete::{self, alpha1, one_of},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use nom::combinator::opt;

#[derive(Clone, Debug)]
enum Op {
    Ins,
    Rem,
}
#[derive(Clone, Debug)]
struct Lens<'a> {
    label: &'a str,
    op: Op,
    focal: Option<u32>,
}
fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    let (_, lenses) = parse_input(input).unwrap();

    let boxes = lenses
        .iter()
        .fold(HashMap::new(), |mut acc: HashMap<u32, Vec<Lens>>, lens| {
            let b = hash(lens.label);
            match acc.get_mut(&b) {
                Some(list) => match lens.op {
                    Op::Ins => match list.iter_mut().find(|l| l.label == lens.label) {
                        Some(l) => {
                            *l = lens.clone();
                        }
                        None => list.push(lens.clone()),
                    },
                    Op::Rem => {
                        if let Some(idx) = list
                            .iter()
                            .enumerate()
                            .find(|(_, l)| l.label == lens.label)
                            .map(|(idx, _)| idx)
                        {
                            let _ = list.remove(idx);
                        }
                    }
                },
                None => {
                    let _ = acc.insert(b, vec![lens.clone()]);
                }
            };
            acc
        });

    boxes
        .iter()
        .map(|(k, v)| -> u32 {
            (k + 1)
                * v.iter()
                    .enumerate()
                    .map(|(idx, l)| {
                        l.focal.expect("Only Lenses with focal go in lists") * (idx as u32 + 1)
                    })
                    .sum::<u32>()
        })
        .sum()
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| ((acc + (c as u32)) * 17) % 256)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Lens>> {
    let (input, l) =
        separated_list1(tag(","), tuple((alpha1, one_of("-="), opt(complete::u32))))(input)?;

    let lenses = l
        .iter()
        .map(|(label, op, focal)| Lens {
            label,
            op: match op {
                '=' => Op::Ins,
                '-' => Op::Rem,
                _ => panic!("invalid Operation {op}"),
            },
            focal: *focal,
        })
        .collect();

    Ok((&input, lenses))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let result = hash("HASH");
        assert_eq!(result, 52);
    }

    #[test]
    fn test_code() {
        let result = solve("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, 145);
    }
}

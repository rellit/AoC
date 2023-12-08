use num::integer::lcm;
use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
struct Node<'a> {
    id: &'a str,
    left: &'a str,
    right: &'a str,
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u64 {
    let directions = parse_map(input).unwrap().1 .0;
    let map = parse_map(input).unwrap().1 .1;

    let steps: Vec<u64> = map
        .values()
        .filter(|node| node.id.ends_with('A'))
        .map(|mut pos| {
            let mut steps: u64 = 0;
            let mut found = false;

            while !found {
                for d in directions.chars() {
                    steps += 1;
                    match d {
                        'R' => pos = map.get(pos.right).expect("Nodes should exist"),
                        'L' => pos = map.get(pos.left).expect("Nodes should exist"),
                        dir => panic!("Direction {dir} unknown"),
                    }
                    if pos.id.ends_with('Z') {
                        found = true;
                        break;
                    }
                }
            }
            steps
        })
        .collect();

    steps.iter().fold(1, |acc, v| lcm(acc, *v))
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let (input, id) = alphanumeric1(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = alphanumeric1(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = alphanumeric1(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((&input, Node { id, left, right }))
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<Node>> {
    let (input, nodes) = separated_list1(tag("\n"), parse_node)(input)?;

    Ok((&input, nodes))
}

fn parse_map(input: &str) -> IResult<&str, (&str, HashMap<&str, Node>)> {
    let (input, (directions, nodes)) = separated_pair(alpha1, tag("\n\n"), parse_nodes)(input)?;

    let map: HashMap<&str, Node> = nodes
        .into_iter()
        .map(|n| (n.id, n))
        .collect::<HashMap<_, _>>();

    Ok((&input, (directions, map)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = solve(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, 6);
    }
}

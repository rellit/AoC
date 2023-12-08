use std::{
    collections::{HashMap},
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

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

fn solve(input: &str) -> u32 {
    let directions = parse_map(input).unwrap().1 .0;
    let map = parse_map(input).unwrap().1 .1;

    let mut steps: u32 = 0;
    let mut found = false;
    let mut pos: &Node = map.get("AAA").expect("Start should exist and be AAA");
    while !found {
        for d in directions.chars() {
            steps += 1;
            match d {
                'R' => pos = map.get(pos.right).expect("Nodes should exist"),
                'L' => pos = map.get(pos.left).expect("Nodes should exist"),
                dir => panic!("Direction {dir} unknown"),
            }
            if pos.id == "ZZZ" {
                found = true;
                break;
            }
        }
    }

    steps
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
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 2);
    }
}

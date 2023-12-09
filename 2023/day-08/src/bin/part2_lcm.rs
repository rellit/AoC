use num::integer::lcm;
use std::collections::HashMap;

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

fn solve(input: &str) -> u64 {
    let (_, (directions, map)) = parse_map(input).expect("Parsable Input");

    let steps: Vec<Vec<u64>> = map
        .values()
        .filter(|node| node.id.ends_with('A'))
        .map(|mut pos| {
            let mut solutions: Vec<u64> = Vec::new();

            let mut steps: u64 = 0;
            let mut index: u64 = 0;

            println!("Search for {pos:?}");
            loop {
                steps += 1;
                let idx = index as usize % directions.len();
                match directions.get(idx..idx + 1) {
                    Some("R") => pos = map.get(pos.right).expect("Nodes should exist"),
                    Some("L") => pos = map.get(pos.left).expect("Nodes should exist"),
                    Some(dir) => panic!("Direction {dir} unknown"),
                    _ => panic!("Invalid direction idx"),
                }
                if pos.id.ends_with('Z') {
                    println!("Found solution {pos:?} {steps:?}");
                    if solutions.iter().filter(|&s| *s == steps).count() > 0 {
                        break;
                    } else {
                        solutions.push(steps);
                        steps = 0;
                    }
                }

                index += 1;
            }
            solutions
        })
        .collect();

    steps
        .iter()
        .flat_map(|s| s.iter()) //Should this be cartesian instead of trying all? I feel like
        .fold(1, |acc, v| lcm(acc, *v))
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

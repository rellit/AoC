use std::{cmp::Ordering, ops::Index};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alphanumeric1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const ORDER: &str = "23456789TJQKA";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Strength {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug)]
struct Draw<'a> {
    cards: &'a str,
    kind: Strength,
    bid: u32,
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    let mut draw = draw(input).unwrap().1;

    draw.sort_by(|d1, d2| match d1.kind.cmp(&d2.kind) {
        std::cmp::Ordering::Equal => compare_cards(d1.cards, d2.cards),
        d => d,
    });

    dbg!(&draw);

    draw.iter()
        .enumerate()
        .map(|(i, d)| d.bid * (i as u32 + 1))
        .sum()
}

fn draw(input: &str) -> IResult<&str, Vec<Draw>> {
    let (input, draw) = separated_list1(
        tag("\n"),
        separated_pair(alphanumeric1, tag(" "), complete::u32),
    )(input)?;

    let draw = draw
        .iter()
        .map(|(cards, bid)| {
            let cards: &str = &cards[0..5];
            Draw {
                cards,
                kind: determine_kind(cards),
                bid: *bid,
            }
        })
        .collect();

    Ok((&input, draw))
}

fn determine_kind(cards: &str) -> Strength {
    let mut pairs: u32 = 0;
    for c in cards.chars() {
        match cards.chars().filter(|char| *char == c).count() {
            5 => return Strength::FiveOfAKind,
            4 => return Strength::FourOfAKind,
            3 => {
                let rest: Vec<char> = cards.chars().filter(|char| *char != c).collect();

                return if rest.get(0).unwrap() == rest.get(1).unwrap() {
                    Strength::FullHouse
                } else {
                    Strength::ThreeOfAKind
                };
            }
            2 => pairs += 1,
            _ => (),
        }
    }

    //Pairs come in groups ;-)
    match pairs {
        4 => Strength::TwoPair,
        2 => Strength::OnePair,
        _ => Strength::HighCard,
    }
}

fn compare_cards(cards1: &str, cards2: &str) -> Ordering {
    for n in 0usize..5usize {
        let c1 = cards1.get(n..n + 1).unwrap();
        let c2 = cards2.get(n..n + 1).unwrap();

        let i1 = ORDER.find(c1).expect("Only valid input");
        let i2 = ORDER.find(c2).expect("Only valid input");

        match i1.cmp(&i2) {
            std::cmp::Ordering::Equal => {
                continue;
            }
            d => {
                return d;
            }
        }
    }
    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = solve(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, 6440);
    }
}

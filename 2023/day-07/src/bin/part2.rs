use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::{self, alphanumeric1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const ORDER: &str = "J23456789TQKA";

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
    let mut draw = parse_draw(input).unwrap().1;

    draw.sort_by(|d1, d2| match d1.kind.cmp(&d2.kind) {
        std::cmp::Ordering::Equal => compare_cards(d1.cards, d2.cards),
        d => d,
    });

    draw.iter()
        .enumerate()
        .map(|(i, d)| d.bid * (i as u32 + 1))
        .sum()
}

fn parse_draw(input: &str) -> IResult<&str, Vec<Draw>> {
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
    let mut strongest = Strength::HighCard;
    for c in cards.chars() {
        match cards
            .chars()
            .filter(|char| *char == c || *char == 'J')
            .count()
        {
            5 => return Strength::FiveOfAKind,
            4 => strongest = strongest.max(Strength::FourOfAKind),
            3 => {
                let rest: Vec<char> = cards
                    .chars()
                    .filter(|char| *char != c && (*char != 'J' || c == 'J'))
                    .collect();

                strongest = strongest.max(if rest.first().unwrap() == rest.get(1).unwrap() {
                    Strength::FullHouse
                } else {
                    Strength::ThreeOfAKind
                });
            }
            2 => pairs += 1,
            _ => (),
        }
    }

    //Could'nt get higher
    if strongest > Strength::TwoPair {
        return strongest;
    }

    //If there is a 'J', and we coul'd come down to here, the answer must be OnePair.
    //TwoPairs, would've been ThreeOfAKind or FullHouse
    //e.g. 1J2J3 or 1J224
    if cards.find(|c| c == 'J').is_some() {
        return Strength::OnePair;
    }

    //Pairs come in groups ;-)
    strongest = strongest.max(match pairs {
        4 => Strength::TwoPair,
        2 => Strength::OnePair,
        _ => Strength::HighCard,
    });
    strongest
}

fn compare_cards(cards1: &str, cards2: &str) -> Ordering {
    for n in 0usize..5usize {
        let c1 = cards1.get(n..n + 1).unwrap();
        let c2 = cards2.get(n..n + 1).unwrap();

        let i1 = ORDER.find(c1).expect("Only valid input");
        let i2 = ORDER.find(c2).expect("Only valid input");

        match i1.cmp(&i2) {
            std::cmp::Ordering::Equal => continue,
            d => return d,
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
        assert_eq!(result, 5905);
    }
}

use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete,
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    points: u32,
}

fn main() {
    let input = include_str!("../input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let games: Vec<Game> = input.lines().map(|line| game(line).unwrap().1).collect();

    let mut map: HashMap<u32, u32> = HashMap::new();

    games.iter().for_each(|game| {
        //Add our self
        if let Some(count) = map.get_mut(&game.id) {
            *count += 1;
        } else {
            map.insert(game.id, 1);
        }

        //All Cards we have won
        for n in game.id..game.id + game.points {
            //If that is a valid card
            if let Some(g) = games.get(n as usize) {
                //For every Card we own with this id
                for _ in 0..*map.get(&game.id).unwrap() {
                    //Add cards we won
                    if let Some(count) = map.get_mut(&g.id) {
                        *count += 1;
                    } else {
                        map.insert(g.id, 1);
                    }
                }
            }
        }
    });

    map.values().sum()
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = take_while1(|c: char| c.is_whitespace())(input)?;
    let (input, game_id) = complete::u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = take_while1(|c: char| c.is_whitespace())(input)?;
    let (input, winning) =
        separated_list1(take_while1(|c: char| c.is_whitespace()), complete::u32)(input)?;
    let (input, _) = take_while1(|c: char| c.is_whitespace())(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = take_while1(|c: char| c.is_whitespace())(input)?;
    let (input, drawn) =
        separated_list1(take_while1(|c: char| c.is_whitespace()), complete::u32)(input)?;

    let points = drawn.iter().filter(|d| winning.contains(d)).count() as u32;

    Ok((
        input,
        Game {
            id: game_id,
            points,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = part2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 30);
    }
}

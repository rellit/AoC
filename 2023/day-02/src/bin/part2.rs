use std::cmp;

#[derive(Debug)]
struct Game {
    _id: u32,
    draws: Vec<Draw>,
}

#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = include_str!("../input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let games = parse_game(input);

    games
        .iter()
        //Get min colors
        .map(|game| {
            game.draws
                .iter()
                .fold((0, 0, 0), |(red, green, blue), draw| {
                    (
                        cmp::max(red, draw.red),
                        cmp::max(green, draw.green),
                        cmp::max(blue, draw.blue),
                    )
                })
        })
        .map(|(red, green, blue)| red * green * blue)
        .sum()
}

fn parse_game(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(":").into_iter();
            let game_id = iter.next().unwrap();

            let draws_str = iter.next().unwrap();
            let draws = draws_str.split(";").into_iter().map(|draw_str| {
                let mut draw = Draw {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                draw_str.split(",").into_iter().for_each(|draw_col| {
                    let mut col = draw_col.trim().split(" ").into_iter();
                    let count: u32 = col.next().unwrap().trim().parse().unwrap();
                    match col.next() {
                        Some("red") => draw.red = count,
                        Some("green") => draw.green = count,
                        Some("blue") => draw.blue = count,
                        _ => {}
                    }
                });

                draw
            });

            Game {
                _id: game_id.replace("Game", "").trim().parse().unwrap(),
                draws: draws.collect(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }
}

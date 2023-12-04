#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input, 12, 13, 14);
    dbg!(output);
}

fn solve(input: &str, red: u32, green: u32, blue: u32) -> u32 {
    let games = parse_game(input);

    games
        .iter()
        //Filter out games with impossible draws
        .filter_map(|game| {
            let mut invalid_draws = game
                .draws
                .iter()
                //Filter out all possible draws
                .filter(|draw| draw.red > red || draw.green > green || draw.blue > blue);

            match invalid_draws.next() {
                None => Some(game.id),
                _ => None,
            }
        })
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
                id: game_id.replace("Game", "").trim().parse().unwrap(),
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
        let result = solve(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            12,
            13,
            14,
        );
        assert_eq!(result, 8);
    }
}

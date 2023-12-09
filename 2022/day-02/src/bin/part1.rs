use std::ops::Add;

enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Add<u32> for Hand {
    type Output = u32;
    fn add(self, other: u32) -> <Self as std::ops::Add<u32>>::Output {
        match self {
            Hand::Rock => 1 + other,
            Hand::Paper => 2 + other,
            Hand::Scissors => 3 + other,
        }
    }
}

struct Round {
    opp: Hand,
    own: Hand,
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|l| {
            let mut s = l.split(' ');
            let opp = s.next()?;
            let own = s.next()?;
            let opp = match opp {
                "A" => Hand::Rock,
                "B" => Hand::Paper,
                "C" => Hand::Scissors,
                h => panic!("Unknown opponent hand: {h}"),
            };
            let own = match own {
                "X" => Hand::Rock,
                "Y" => Hand::Paper,
                "Z" => Hand::Scissors,
                h => panic!("Unknown opponent hand: {h}"),
            };
            Some(Round { opp, own })
        })
        .map(|h| points_for(&h))
        .sum()
}

fn points_for(r: &Round) -> u32 {
    use Hand::*;
    match r.own {
        Rock => {
            Rock + match r.opp {
                Rock => 3,
                Paper => 0,
                Scissors => 6,
            }
        }
        Scissors => {
            Scissors
                + match r.opp {
                    Rock => 0,
                    Paper => 6,
                    Scissors => 3,
                }
        }
        Paper => {
            Paper
                + match r.opp {
                    Rock => 6,
                    Paper => 3,
                    Scissors => 0,
                }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = solve(
            "A Y
B X
C Z",
        );
        assert_eq!(result, 15);
    }
}

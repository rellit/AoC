use std::ops::Add;

enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Loose,
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

impl Add<Hand> for u32 {
    type Output = u32;
    fn add(self, other: Hand) -> <Self as std::ops::Add<u32>>::Output {
        match other {
            Hand::Rock => 1 + self,
            Hand::Paper => 2 + self,
            Hand::Scissors => 3 + self,
        }
    }
}

struct Round {
    opp: Hand,
    outcome: Outcome,
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
            let outcome = s.next()?;
            let opp = match opp {
                "A" => Hand::Rock,
                "B" => Hand::Paper,
                "C" => Hand::Scissors,
                h => panic!("Unknown opponent hand: {h}"),
            };
            let outcome = match outcome {
                "X" => Outcome::Loose,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                h => panic!("Unknown outcome: {h}"),
            };
            Some(Round { opp, outcome })
        })
        .map(|h| points_for(&h))
        .sum()
}

fn points_for(r: &Round) -> u32 {
    use Hand::*;
    use Outcome::*;
    match r.outcome {
        Loose => {
            0 + match r.opp {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            }
        }
        Draw => {
            3 + match r.opp {
                Rock => Rock,
                Paper => Paper,
                Scissors => Scissors,
            }
        }
        Win => {
            6 + match r.opp {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
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
        assert_eq!(result, 12);
    }
}

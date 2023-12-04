fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    let mut carry = input
        .lines()
        .fold(Vec::<Vec<usize>>::new(), |mut acc, line| {
            let l = acc.len();

            if let Ok(num) = line.parse::<usize>() {
                if let Some(vec) = acc.get_mut((l).saturating_sub(1)) {
                    vec.push(num)
                } else {
                    acc.push(vec![num])
                }
            } else {
                acc.push(Vec::new())
            }

            acc
        });

        carry.sort_by(|c1, c2| c2.iter().sum::<usize>().cmp(&c1.iter().sum::<usize>()) );
    
    carry.iter().take(3).map(|c|c.iter().sum::<usize>()).sum::<usize>()  as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = solve(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );
        assert_eq!(result, 45000);
    }
}

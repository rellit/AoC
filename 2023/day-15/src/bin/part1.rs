fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    let input = parse_input(input);

    input.iter().map(|s| hash(s)).sum()
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| ((acc + (c as u32)) * 17) % 256)
}

fn parse_input(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let result = hash("HASH");
        assert_eq!(result, 52);
    }

    #[test]
    fn test_code() {
        let result = solve("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, 1320);
    }
}

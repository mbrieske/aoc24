use itertools::Itertools;
advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
enum SequenceSafety {
    SafeUp,
    SafeDown,
    Unsafe,
}

fn is_safe(seq: &[u32]) -> bool {
    let seq_safety: Vec<SequenceSafety> = seq
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| match a as i64 - b as i64 {
            1..=3 => SequenceSafety::SafeUp,
            -3..=-1 => SequenceSafety::SafeDown,
            _ => SequenceSafety::Unsafe,
        })
        .collect();
    let first = seq_safety.first().unwrap();
    first != &SequenceSafety::Unsafe && seq_safety.iter().all(|s| s == first)
}

fn can_be_made_safe(seq: &[u32]) -> bool {
    for i in 0..seq.len() {
        let test_seq = seq[..i].iter().chain(seq[i + 1..].iter());
        if is_safe(test_seq.copied().collect::<Vec<u32>>().as_slice()) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let seq: Vec<u32> = line
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                is_safe(seq.as_slice())
            })
            .filter(|&s| s)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let seq: Vec<u32> = line
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                is_safe(seq.as_slice()) || can_be_made_safe(seq.as_slice())
            })
            .filter(|&s| s)
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(2))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(4))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

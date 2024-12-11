use cached::proc_macro::cached;

advent_of_code::solution!(11);

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

#[cached]
fn blink(n: u64, blinks: u8) -> u64 {
    if blinks == 0 {
        1
    } else {
        if n == 0 {
            blink(1, blinks - 1)
        } else if n.to_string().chars().count() % 2 == 0 {
            let nstr = n.to_string();
            let mid = nstr.len() / 2;
            blink(nstr[..mid].parse::<u64>().unwrap(), blinks - 1)
                + blink(nstr[mid..].parse::<u64>().unwrap(), blinks - 1)
        } else {
            blink(n * 2024, blinks - 1)
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let numbers = parse(input);
    Some(numbers.iter().map(|&n| blink(n, 25)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let numbers = parse(input);
    Some(numbers.iter().map(|&n| blink(n, 75)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(55312))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u64>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), None)]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u64>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

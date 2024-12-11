advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut numbers = input
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    (0..25).for_each(|_| {
        numbers = numbers
            .iter()
            .flat_map(|&n| {
                if n == 0 {
                    vec![1 as u64]
                } else if n.to_string().chars().count() % 2 == 0 {
                    let nstr = n.to_string();
                    let mid = nstr.len() / 2;
                    vec![
                        nstr[..mid].parse::<u64>().unwrap(),
                        nstr[mid..].parse::<u64>().unwrap(),
                    ]
                } else {
                    vec![n * 2024]
                }
            })
            .collect();
    });
    Some(numbers.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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

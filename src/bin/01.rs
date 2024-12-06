advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|l| {
            l.split_once("   ")
                .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        })
        .unzip();

    left.sort();
    right.sort();

    Some(
        left.into_iter()
            .zip(right)
            .fold(0, |acc, (l, r)| acc + l.abs_diff(r)),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|l| {
            l.split_once("   ")
                .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        })
        .unzip();

    Some(left.iter().fold(0, |acc, l| {
        acc + l * right.iter().filter(|&r| r == l).count() as u32
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(11))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::DEBUG);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(31))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(re.captures_iter(input).map(|cap| {
        let a = cap[1].parse::<u32>().unwrap();
        let b = cap[2].parse::<u32>().unwrap();
        a * b
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don\'t\(\))").unwrap();
    Some(re.captures_iter(input).fold((true, 0), |acc, cap| {
        match &cap[0] {
            "do()" => (true, acc.1),
            "don't()" => (false, acc.1),
            _ => {
                let a = cap[1].parse::<u32>().unwrap();
                let b = cap[2].parse::<u32>().unwrap();
                (acc.0, acc.1 + if acc.0 { a * b } else { 0 })
            }
        }
    }).1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(161))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(48))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

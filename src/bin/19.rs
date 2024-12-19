use cached::proc_macro::cached;
advent_of_code::solution!(19);

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    (towels, lines.skip(1).collect())
}

fn can_display(towels: &[&str], design: &str) -> bool {
    towels.iter().any(|&towel| {
        if let Some(rest) = design.strip_prefix(towel) {
            if rest.is_empty() {
                true
            } else {
                can_display(towels, rest)
            }
        } else {
            false
        }
    })
}

#[cached(key = "String", convert = r#"{ design.to_string() }"#)]
fn ways_to_display(towels: &[&str], design: &str) -> u64 {
    towels.iter().fold(0, |acc, &towel| {
        if let Some(rest) = design.strip_prefix(towel) {
            if rest.is_empty() {
                acc + 1
            } else {
                acc + ways_to_display(towels, rest)
            }
        } else {
            acc
        }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let (towels, designs) = parse(input);
    let res = designs
        .iter()
        .filter(|design| can_display(&towels, design))
        .count() as u64;
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (towels, designs) = parse(input);
    let res = designs
        .iter()
        .map(|design| ways_to_display(&towels, design))
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(6))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u64>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(16))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u64>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

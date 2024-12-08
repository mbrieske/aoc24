advent_of_code::solution!(7);

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
    Merge,
}

fn can_be_made_true_with_merge(result: u64, numbers: &[u64]) -> bool {
    let operator_combinations = 3_u64.pow((numbers.len() - 1) as u32);
    let possibly_true = (0..operator_combinations)
        .map(|combination| {
            let mut operators = (0..numbers.len() - 1)
                .map(|i| match combination / 3_u64.pow(i as u32) % 3 {
                    0 => Operation::Add,
                    1 => Operation::Multiply,
                    2 => Operation::Merge,
                    _ => panic!(),
                })
                .collect::<Vec<Operation>>();
            operators.insert(0, Operation::Add); // first number is always added
            operators
                .into_iter()
                .zip(numbers.iter())
                .try_fold(0, |acc, (op, &n)| {
                    let new_acc = match op {
                        Operation::Add => acc + n,
                        Operation::Multiply => acc * n,
                        Operation::Merge => {
                            (acc.to_string() + &n.to_string()).parse::<u64>().unwrap()
                        }
                    };
                    if new_acc > result {
                        Err(new_acc)
                    } else {
                        Ok(new_acc)
                    }
                })
        })
        .filter_map(|result| result.ok())
        .any(|r| r == result);
    possibly_true
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>());
    let equations = equations.map(|eq| {
        let (&result, numbers) = eq.split_first().unwrap();
        let result = result
            .strip_suffix(|_: char| true)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        (
            result,
            numbers
                .iter()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        )
    });
    let sum: u64 = equations
        .map(|(result, numbers)| {
            let operator_combinations = 2_u64.pow((numbers.len() - 1) as u32);
            let possibly_true = (0..operator_combinations)
                .map(|combination| {
                    let mut numbers = numbers.iter();
                    let first = *numbers.next().unwrap();
                    numbers.enumerate().try_fold(first, |acc, (i, &n)| {
                        let operation = if combination & (1 << i) == 0 {
                            Operation::Add
                        } else {
                            Operation::Multiply
                        };
                        let new_acc = match operation {
                            Operation::Add => acc + n,
                            Operation::Multiply => acc * n,
                            Operation::Merge => panic!(), // only relevant for part 2
                        };
                        if new_acc > result {
                            Err(new_acc)
                        } else {
                            Ok(new_acc)
                        }
                    })
                })
                .filter_map(|result| result.ok())
                .any(|r| r == result);
            if possibly_true {
                result
            } else {
                0
            }
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>());
    let equations = equations.map(|eq| {
        let (&result, numbers) = eq.split_first().unwrap();
        let result = result
            .strip_suffix(|_: char| true)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        (
            result,
            numbers
                .iter()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        )
    });

    let sum: u64 = equations
        .map(|(result, numbers)| {
            if can_be_made_true_with_merge(result, &numbers) {
                result
            } else {
                0
            }
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(3749))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u64>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(11387))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u64>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(7290, vec![6, 8, 6, 15], true)]
    #[case(192, vec![17, 8, 14], true)]
    fn test_can_be_made_true_part_two(
        #[case] result: u64,
        #[case] numbers: Vec<u64>,
        #[case] expected: bool,
    ) {
        let result = can_be_made_true_with_merge(result, &numbers);
        assert_eq!(result, expected);
    }
}

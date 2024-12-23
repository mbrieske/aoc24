use std::collections::HashMap;

advent_of_code::solution!(22);

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn next(mut a: usize) -> usize {
    a = prune(mix(a, a << 6)); // a * 64
    a = prune(mix(a, a >> 5)); // a / 32
    a = prune(mix(a, a << 11)); // a * 2048
    a
}

fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

fn prune(a: usize) -> usize {
    a & 0xFFFFFF // mod 16777216
}

pub fn part_one(input: &str) -> Option<usize> {
    let initial_nbrs = parse(input);
    initial_nbrs
        .into_iter()
        .map(|nbr| {
            let mut a = nbr;
            for _ in 0..2000 {
                a = next(a);
            }
            a
        })
        .sum::<usize>()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let initial_nbrs = parse(input);
    let sequences = initial_nbrs
        .into_iter()
        .map(|nbr| {
            let mut sequences_singlebuyer = HashMap::<[i8; 4], usize>::new();
            let v: Vec<_> = std::iter::successors(Some(nbr), |prev| Some(next(*prev)))
                .map(|a| a % 10)
                .take(2001)
                .collect();

            let deltas: Vec<_> = v
                .windows(2)
                .map(|vw| (vw[1] as u8, (vw[1] as i64 - vw[0] as i64) as i8))
                .collect();

            deltas.windows(4).for_each(|seq| {
                let ones_seq: [i8; 4] = seq
                    .iter()
                    .map(|(_, d)| *d)
                    .collect::<Vec<i8>>()
                    .try_into()
                    .unwrap();
                sequences_singlebuyer
                    .entry(ones_seq)
                    .or_insert(seq[3].0 as usize);
            });
            sequences_singlebuyer
        })
        .fold(HashMap::new(), |mut acc, map| {
            for (key, value) in map {
                *acc.entry(key).or_insert(0) += value;
            }
            acc
        });
    let max_bananas = sequences.iter().max_by_key(|(_, d)| *d).unwrap().1;
    Some(*max_bananas)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[test]
    fn test_next() {
        assert_eq!(next(123), 15887950);
    }

    #[test]
    fn test_2000() {
        let mut a = 1;
        for _ in 0..2000 {
            a = next(a);
        }
        assert_eq!(a, 8685429);
    }

    #[test]
    fn test_10() {
        let nbr = 123;
        let v: Vec<_> = std::iter::successors(Some(nbr), |prev| Some(next(*prev)))
            .take(10)
            .collect();
        dbg!(v);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(37327623))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<usize>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(23))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<usize>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

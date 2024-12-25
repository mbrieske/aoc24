advent_of_code::solution!(25);

type Lock = [u8; 5];
type Key = [u8; 5];

enum Schematic {
    Lock(Lock),
    Key(Key),
}

impl Schematic {
    fn from(lines: &[&str]) -> Self {
        if lines[0].starts_with("#") {
            let pins = lines.iter().skip(1).fold([0; 5], |mut acc, &line| {
                line.chars().enumerate().for_each(|(i, c)| {
                    if c == '#' {
                        acc[i] += 1;
                    }
                });
                acc
            });
            Self::Lock(pins)
        } else {
            let pins = lines.iter().rev().skip(1).fold([0; 5], |mut acc, &line| {
                line.chars().enumerate().for_each(|(i, c)| {
                    if c == '#' {
                        acc[i] += 1;
                    }
                });
                acc
            });
            Self::Key(pins)
        }
    }
}

fn matches(lock: &Lock, key: &Key) -> bool {
    lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 5)
}

fn parse(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let schematics = input
        .split("\n\n")
        .map(|s| Schematic::from(&s.lines().collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    let mut locks = Vec::new();
    let mut keys = Vec::new();
    schematics
        .into_iter()
        .for_each(|schematic| match schematic {
            Schematic::Lock(lock) => locks.push(lock),
            Schematic::Key(key) => keys.push(key),
        });
    (locks, keys)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (locks, keys) = parse(input);
    locks
        .iter()
        .map(|lock| keys.iter().filter(|key| matches(lock, key)).count() as u32)
        .sum::<u32>()
        .into()
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(3))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }
}

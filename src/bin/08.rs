use glam::IVec2;
use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(8);

fn parse_input(input: &str) -> (usize, usize, HashMap<char, Vec<IVec2>>) {
    let mut lines = input.lines();
    let map_width = lines.next().unwrap().len();
    let map_heigth = lines.count() + 1;

    let antennas: HashMap<char, Vec<IVec2>> = input
        .chars()
        .filter(|c| c != &'\n')
        .enumerate()
        .filter(|(_, c)| *c != '.')
        .fold(HashMap::new(), |mut map, (i, antenna)| {
            map.entry(antenna).or_default().push(IVec2 {
                x: (i % map_width) as i32,
                y: (i / map_width) as i32,
            });
            map
        });

    (map_width, map_heigth, antennas)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map_width, map_heigth, antennas) = parse_input(input);
    let num_antinodes = antennas
        .iter()
        .flat_map(|(_, positions)| {
            positions.iter().combinations(2).flat_map(|pair| {
                let (a, b) = (pair[0], pair[1]);
                let delta = b - a;
                vec![a - delta, b + delta]
            })
        })
        .unique()
        .filter(|&p| p.x >= 0 && p.x < map_width as i32 && p.y >= 0 && p.y < map_heigth as i32)
        .count() as u32;
    Some(num_antinodes)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map_width, map_heigth, antennas) = parse_input(input);
    let num_antinodes = antennas
        .iter()
        .flat_map(|(_, positions)| {
            positions.iter().combinations(2).flat_map(|pair| {
                let (a, b) = (pair[0], pair[1]);
                let delta = b - a;
                let mut antinodes = vec![*a, *b];

                let mut antinode_candidate = a - delta;
                while antinode_candidate.x >= 0
                    && antinode_candidate.x < map_width as i32
                    && antinode_candidate.y >= 0
                    && antinode_candidate.y < map_heigth as i32
                {
                    antinodes.push(antinode_candidate);
                    antinode_candidate -= delta;
                }

                antinode_candidate = b + delta;
                while antinode_candidate.x >= 0
                    && antinode_candidate.x < map_width as i32
                    && antinode_candidate.y >= 0
                    && antinode_candidate.y < map_heigth as i32
                {
                    antinodes.push(antinode_candidate);
                    antinode_candidate += delta;
                }

                antinodes
            })
        })
        .unique()
        .filter(|&p| p.x >= 0 && p.x < map_width as i32 && p.y >= 0 && p.y < map_heigth as i32)
        .count() as u32;
    Some(num_antinodes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(14))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(34))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

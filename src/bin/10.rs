use glam::IVec2;
use pathfinding::prelude::bfs_reach;
use std::collections::HashMap;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    pos: IVec2,
    value: u8,
}

fn parse(input: &str) -> (Vec<IVec2>, HashMap<IVec2, Node>) {
    input.lines().enumerate().fold(
        (Vec::new(), HashMap::new()),
        |(mut start_positions, mut graph), (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let value = c.to_digit(10).unwrap() as u8;
                let pos = IVec2::new(x as i32, y as i32);
                let node = Node { pos, value };
                if value == 0 {
                    start_positions.push(pos);
                }
                graph.insert(pos, node);
            });
            (start_positions, graph)
        },
    )
}

fn successors<'a>(graph: &'a HashMap<IVec2, Node>, n: &'a Node) -> Vec<&'a Node> {
    [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
        .iter()
        .map(|dir| n.pos + dir)
        .filter_map(|pos| graph.get(&pos))
        .filter(|s| s.value == n.value + 1)
        .collect()
}

fn successor_paths<'a>(graph: &'a HashMap<IVec2, Node>, n: Vec<&'a Node>) -> Vec<Vec<&'a Node>> {
    successors(graph, n.last().unwrap())
        .into_iter()
        .map(|s| {
            let mut n = n.clone();
            n.push(s);
            n
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start_positions, graph) = parse(input);

    let res: u32 = start_positions
        .iter()
        .map(|&pos| {
            bfs_reach(graph.get(&pos).unwrap(), |n| successors(&graph, n))
                .filter(|n| n.value == 9)
                .count() as u32
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start_positions, graph) = parse(input);

    let res = start_positions
        .iter()
        .map(|&pos| {
            let start = vec![graph.get(&pos).unwrap()];
            bfs_reach(start, |n| successor_paths(&graph, n.clone()))
                .filter(|n| n.last().unwrap().value == 9)
                .count() as u32
        })
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
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(36))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(81))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

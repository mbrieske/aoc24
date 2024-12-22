use advent_of_code::AocGrid;
use glam::IVec2;
use grid::Grid;
use pathfinding::prelude::bfs;

advent_of_code::solution!(20);

fn successors(pos: &IVec2, grid: &Grid<char>) -> Vec<IVec2> {
    grid.neighbors(*pos)
        .filter(|(_, &n)| n != '#')
        .map(|(p, _)| p)
        .collect()
}

fn manhattan_distance(a: IVec2, b: IVec2) -> usize {
    (a - b).abs().element_sum() as usize
}

fn solve(input: &str, min_save: usize, max_cheat_picos: usize) -> Option<u32> {
    let maze = Grid::<char>::from_input(input);
    let start = maze
        .indexed_iter()
        .find(|(_, c)| **c == 'S')
        .map(|((y, x), _)| IVec2::new(x as i32, y as i32))
        .unwrap();

    let finish = maze
        .indexed_iter()
        .find(|(_, c)| **c == 'E')
        .map(|((y, x), _)| IVec2::new(x as i32, y as i32))
        .unwrap();

    let path = bfs(&start, |p| successors(p, &maze), |&p| p == finish).unwrap();

    let cheats =
        path[..path.len() - min_save]
            .iter()
            .enumerate()
            .flat_map(|(from_idx, from_pos)| {
                path[from_idx + min_save..]
                    .iter()
                    .enumerate()
                    .map(move |(i, to_pos)| {
                        (
                            from_idx,
                            i + from_idx + min_save, // to_idx
                            manhattan_distance(*to_pos, *from_pos),
                        )
                    })
                    .filter(|&(from_idx, to_idx, dist)| {
                        let steps = to_idx - from_idx;
                        let saving = steps - dist;
                        dist <= max_cheat_picos && saving >= min_save
                    })
            });
    Some(cheats.count() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 100, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 100, 20)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(5))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = solve(input, 20, 2);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(7))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = solve(input, 74, 20);
        assert_eq!(result, expected);
    }
}

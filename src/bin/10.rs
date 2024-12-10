use glam::IVec2;
use grid::Grid;
use pathfinding::prelude::bfs_reach;

advent_of_code::solution!(10);

trait IGrid {
    fn iget(&self, pos: IVec2) -> Option<&u8>;
}

impl IGrid for Grid<u8> {
    fn iget(&self, pos: IVec2) -> Option<&u8> {
        self.get(usize::try_from(pos.y).ok()?, usize::try_from(pos.x).ok()?)
    }
}

fn parse(input: &str) -> (Vec<IVec2>, Grid<u8>) {
    let mut start_positions = Vec::new();
    let width = input.lines().next().unwrap().chars().count();
    let iv: Vec<u8> = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c.to_digit(10).unwrap() as u8)
        .enumerate()
        .inspect(|(i, v)| {
            if *v == 0 {
                start_positions.push(IVec2::new((i % width) as i32, (i / width) as i32))
            }
        })
        .map(|(_, v)| v)
        .collect();

    let grid = Grid::from_vec(iv, width);

    (start_positions, grid)
}

fn successors(grid: &Grid<u8>, pos: IVec2, v: u8) -> Vec<(IVec2, u8)> {
    [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
        .iter()
        .map(|dir| pos + dir)
        .filter_map(|next_pos| grid.iget(next_pos).map(|&next_val| (next_pos, next_val)))
        .filter(|&(_, s)| s == v + 1)
        .collect()
}

fn successor_paths(grid: &Grid<u8>, path: Vec<IVec2>, v: u8) -> Vec<(Vec<IVec2>, u8)> {
    let last = path.last().unwrap();
    successors(grid, *last, v)
        .into_iter()
        .map(|(s_pos, s_val)| {
            let mut path = path.clone();
            path.push(s_pos);
            (path, s_val)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start_positions, grid) = parse(input);

    let res: u32 = start_positions
        .iter()
        .map(|&pos| {
            let v_first = *grid.iget(pos).unwrap();
            bfs_reach((pos, v_first), |&(n_pos, n_val)| {
                successors(&grid, n_pos, n_val)
            })
            .filter(|&(_, n_val)| n_val == 9)
            .count() as u32
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start_positions, grid) = parse(input);

    let res = start_positions
        .iter()
        .map(|&pos| {
            let start = (vec![pos], *grid.iget(pos).unwrap());
            bfs_reach(start, |(path, v)| successor_paths(&grid, path.clone(), *v))
                .filter(|&(_, n_val)| n_val == 9)
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

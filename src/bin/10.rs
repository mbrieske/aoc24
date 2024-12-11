use advent_of_code::AocGrid;
use glam::IVec2;
use grid::Grid;
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

trait TopoGrid {
    fn next(&self, pos: IVec2, v: u8) -> Vec<(IVec2, u8)>;
}

impl TopoGrid for Grid<u8> {
    fn next(&self, pos: IVec2, v: u8) -> Vec<(IVec2, u8)> {
        [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
            .iter()
            .map(|dir| pos + dir)
            .filter_map(|next_pos| {
                self.get_ivec(next_pos)
                    .map(|&next_val| (next_pos, next_val))
            })
            .filter(|&(_, s)| s == v + 1)
            .collect()
    }
}

fn parse(input: &str) -> (Vec<IVec2>, Grid<u8>) {
    let grid: Grid<u8> = Grid::from_input(input);

    let start_positions = grid
        .indexed_iter()
        .filter(|(_, &v)| v == 0)
        .map(|(pos, _)| IVec2::new(pos.1 as i32, pos.0 as i32))
        .collect();

    (start_positions, grid)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start_positions, grid) = parse(input);

    let res: u32 = start_positions
        .iter()
        .map(|&pos| {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((pos, 0));
            while !queue.is_empty() {
                let (pos, v) = queue.pop_front().unwrap();
                if v == 9 {
                    visited.insert(pos);
                } else {
                    for (next_pos, next_val) in grid.next(pos, v) {
                        queue.push_back((next_pos, next_val));
                    }
                }
            }
            visited.len() as u32
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start_positions, grid) = parse(input);

    let res: u32 = start_positions
        .iter()
        .map(|&pos| {
            let mut trail_count = 0;
            let mut queue = VecDeque::new();
            queue.push_back((pos, 0));
            while !queue.is_empty() {
                let (pos, v) = queue.pop_front().unwrap();
                if v == 9 {
                    trail_count += 1;
                } else {
                    for (next_pos, next_val) in grid.next(pos, v) {
                        queue.push_back((next_pos, next_val));
                    }
                }
            }
            trail_count
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

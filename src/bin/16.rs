use advent_of_code::AocGrid;
use glam::IVec2;
use grid::Grid;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_ivec(ivec: IVec2) -> Self {
        match ivec {
            IVec2::X => Direction::Right,
            IVec2::Y => Direction::Down,
            IVec2::NEG_X => Direction::Left,
            IVec2::NEG_Y => Direction::Up,
            _ => panic!("Invalid direction"),
        }
    }

    fn ivec(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::NEG_Y,
            Direction::Down => IVec2::Y,
            Direction::Left => IVec2::NEG_X,
            Direction::Right => IVec2::X,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pos: IVec2,
    dir: Direction,
}

impl Pos {
    fn successors(&self, grid: &Grid<char>) -> Vec<(Pos, usize)> {
        // can only move straight or change direction 90deg
        let mut successors = Vec::new();
        let step = match self.dir {
            Direction::Up => IVec2::NEG_Y,
            Direction::Down => IVec2::Y,
            Direction::Left => IVec2::NEG_X,
            Direction::Right => IVec2::X,
        };
        let step_pos = self.pos + step;
        if let Some(c) = grid.get_ivec(step_pos) {
            if *c != '#' {
                successors.push((
                    Pos {
                        pos: step_pos,
                        dir: self.dir,
                    },
                    1,
                ));
            }
        }

        let dir_ivec = self.dir.ivec();
        // (-y, x) is clockwise and (y, -x) is counterclockwise

        successors.push((
            Pos {
                pos: self.pos,
                dir: Direction::from_ivec(IVec2::new(-dir_ivec.y, dir_ivec.x)),
            },
            1000,
        ));
        successors.push((
            Pos {
                pos: self.pos,
                dir: Direction::from_ivec(IVec2::new(dir_ivec.y, -dir_ivec.x)),
            },
            1000,
        ));
        successors
    }
}

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::<char>::from_input(input);
    let start = grid
        .indexed_iter()
        .find(|(_, c)| **c == 'S')
        .map(|((y, x), _)| Pos {
            pos: IVec2::new(x as i32, y as i32),
            dir: Direction::Right,
        })
        .unwrap();
    let finish = grid
        .indexed_iter()
        .find(|(_, c)| **c == 'E')
        .map(|((y, x), _)| IVec2::new(x as i32, y as i32))
        .unwrap();

    let result = dijkstra(&start, |p| p.successors(&grid), |p| p.pos == finish);
    Some(result.unwrap().1 as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::<char>::from_input(input);
    let start = grid
        .indexed_iter()
        .find(|(_, c)| **c == 'S')
        .map(|((y, x), _)| Pos {
            pos: IVec2::new(x as i32, y as i32),
            dir: Direction::Right,
        })
        .unwrap();
    let finish = grid
        .indexed_iter()
        .find(|(_, c)| **c == 'E')
        .map(|((y, x), _)| IVec2::new(x as i32, y as i32))
        .unwrap();

    let (path, _) = dijkstra(&start, |p| p.successors(&grid), |p| p.pos == finish).unwrap();
    Some(path.iter().map(|p| p.pos).unique().count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(7036))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(11048))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(45))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(64))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

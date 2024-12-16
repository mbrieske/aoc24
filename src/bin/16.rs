use advent_of_code::AocGrid;
use glam::IVec2;
use grid::Grid;
use itertools::Itertools;
use pathfinding::prelude::{dijkstra, dijkstra_all};
use std::{collections::HashMap, hash::Hash};

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
        let mut successors = Vec::new();
        let forward = self.pos + self.dir.ivec();
        // Move forward
        if let Some(c) = grid.get_ivec(forward) {
            if *c != '#' {
                successors.push((
                    Pos {
                        pos: forward,
                        dir: self.dir,
                    },
                    1,
                ));
            }
        }

        // Turn left or right
        let dir_ivec = self.dir.ivec();
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

    fn predecessors(&self, grid: &Grid<char>) -> Vec<(Pos, usize)> {
        let mut preds = Vec::new();
        let forward = self.pos - self.dir.ivec();
        if let Some(c) = grid.get_ivec(forward) {
            if *c != '#' {
                preds.push((
                    Pos {
                        pos: forward,
                        dir: self.dir,
                    },
                    1,
                ));
            }
        }

        // Turn left or right
        let dir_ivec = self.dir.ivec();
        preds.push((
            Pos {
                pos: self.pos,
                dir: Direction::from_ivec(IVec2::new(-dir_ivec.y, dir_ivec.x)),
            },
            1000,
        ));
        preds.push((
            Pos {
                pos: self.pos,
                dir: Direction::from_ivec(IVec2::new(dir_ivec.y, -dir_ivec.x)),
            },
            1000,
        ));

        preds
    }
}

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::<char>::from_input(input);

    let start_pos = grid
        .indexed_iter()
        .find(|(_, c)| **c == 'S')
        .map(|((y, x), _)| IVec2::new(x as i32, y as i32))
        .unwrap();

    let finish_pos = grid
        .indexed_iter()
        .find(|(_, c)| **c == 'E')
        .map(|((y, x), _)| IVec2::new(x as i32, y as i32))
        .unwrap();

    let start = Pos {
        pos: start_pos,
        dir: Direction::Right,
    };

    let result = dijkstra(&start, |p| p.successors(&grid), |p| p.pos == finish_pos);
    Some(result.unwrap().1 as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::<char>::from_input(input);

    let start_pos = grid
        .indexed_iter()
        .find(|(_, c)| **c == 'S')
        .map(|((y, x), _)| IVec2::new(x as i32, y as i32))
        .unwrap();

    let finish_pos = grid
        .indexed_iter()
        .find(|(_, c)| **c == 'E')
        .map(|((y, x), _)| IVec2::new(x as i32, y as i32))
        .unwrap();

    let start = Pos {
        pos: start_pos,
        dir: Direction::Right,
    };

    let dist_from_start_map = {
        let mut dist_map = dijkstra_all(&start, |p| p.successors(&grid));
        dist_map.insert(start, (start, 0));
        dist_map
    };

    let shortest_finish_dist = dist_from_start_map
        .iter()
        .filter_map(|(p, &(_, cost))| {
            if p.pos == finish_pos {
                Some(cost)
            } else {
                None
            }
        })
        .min()?;

    let mut dist_from_finish: HashMap<Pos, usize> = HashMap::new();

    dist_from_start_map
        .iter()
        .filter(|(p, &(_, cost))| p.pos == finish_pos && cost == shortest_finish_dist)
        .map(|(&finish_state, _)| {
            let mut dist_map = dijkstra_all(&finish_state, |p| p.predecessors(&grid));
            dist_map.insert(finish_state, (finish_state, 0));
            dist_map
        })
        .for_each(|candidate_map| {
            for (p, &(_parent, cost)) in &candidate_map {
                let e = dist_from_finish.entry(*p).or_insert(cost);
                if cost < *e {
                    *e = cost;
                }
            }
        });

    let dist_from_start: HashMap<Pos, usize> = dist_from_start_map
        .iter()
        .map(|(p, &(_, cost))| (*p, cost))
        .collect();

    let tiles_on_path = dist_from_start
        .iter()
        .filter(|&(p, &cost_start)| {
            let cost_finish = dist_from_finish.get(p).unwrap();
            cost_start + cost_finish == shortest_finish_dist
        })
        .map(|(p, _)| p.pos)
        .unique();

    Some(tiles_on_path.count() as u32)
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

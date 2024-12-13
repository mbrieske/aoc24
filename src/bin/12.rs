use std::collections::HashSet;

use advent_of_code::AocGrid;
use glam::IVec2;
use grid::Grid;

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Fences where inside is NORTH or SOUTH go from start in positive X direction,
// fences where inside is EAST or WEST go from start in positive Y direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Fence {
    start: IVec2,
    inside: Direction,
}

fn grow_region(
    grid: &Grid<char>,
    pos: IVec2,
    rtype: char,
    seen: &mut HashSet<IVec2>,
    region: &mut Vec<IVec2>,
) {
    if seen.contains(&pos) {
        return;
    }

    seen.insert(pos);
    region.push(pos);

    grid.neighbors(pos)
        .filter(|(_, &nrtype)| nrtype == rtype)
        .for_each(|(npos, _)| grow_region(grid, npos, rtype, seen, region));
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid<char> = Grid::from_input(input);
    let mut seen: HashSet<IVec2> = HashSet::new();
    let mut regions: Vec<(Vec<IVec2>, char)> = Vec::new();

    grid.indexed_iter()
        .map(|(pos, c)| (IVec2::new(pos.1 as i32, pos.0 as i32), c))
        .for_each(|(pos, &c)| {
            if seen.contains(&pos) {
                return;
            }

            let mut region: Vec<IVec2> = Vec::new();
            grow_region(&grid, pos, c, &mut seen, &mut region);
            regions.push((region, c));
        });

    let res: u32 = regions
        .iter()
        .map(|(region, rtype)| {
            region
                .iter()
                .map(|pos| {
                    4 - grid
                        .neighbors(*pos)
                        .filter(|(_, &nrtype)| nrtype == *rtype)
                        .count() as u32
                })
                .sum::<u32>()
                * region.len() as u32
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Grid<char> = Grid::from_input(input);
    let mut seen: HashSet<IVec2> = HashSet::new();
    let mut regions: Vec<(Vec<IVec2>, char)> = Vec::new();

    grid.indexed_iter()
        .map(|(pos, c)| (IVec2::new(pos.1 as i32, pos.0 as i32), c))
        .for_each(|(pos, &c)| {
            if seen.contains(&pos) {
                return;
            }

            let mut region: Vec<IVec2> = Vec::new();
            grow_region(&grid, pos, c, &mut seen, &mut region);
            regions.push((region, c));
        });

    let res: u32 = regions
        .iter()
        .map(|(region, _)| {
            let fences: HashSet<Fence> = region
                .iter()
                .flat_map(|pos| {
                    let neighbors = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                        .iter()
                        .filter(move |dir| {
                            let ncand = pos + *dir;
                            !region.contains(&ncand)
                        });
                    neighbors.map(move |dir| match *dir {
                        IVec2::X => Fence {
                            start: pos + IVec2::new(1, 0),
                            inside: Direction::West,
                        },
                        IVec2::Y => Fence {
                            start: pos + IVec2::new(0, 1),
                            inside: Direction::South,
                        },
                        IVec2::NEG_X => Fence {
                            start: *pos,
                            inside: Direction::East,
                        },
                        IVec2::NEG_Y => Fence {
                            start: *pos,
                            inside: Direction::North,
                        },
                        _ => panic!(),
                    })
                })
                .collect();

            let fence_starts = fences.iter().filter(|f| {
                let before_pos = match f.inside {
                    Direction::North | Direction::South => f.start - IVec2::X,
                    Direction::East | Direction::West => f.start - IVec2::Y,
                };

                let before = Fence {
                    start: before_pos,
                    inside: f.inside,
                };
                !fences.contains(&before)
            });
            fence_starts.count() as u32 * region.len() as u32
        })
        .sum::<u32>();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(140))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(772))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 3), Some(1930))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(80))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(436))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 4), Some(16))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 5), Some(236))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 6), Some(368))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

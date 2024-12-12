use std::collections::HashSet;

use advent_of_code::AocGrid;
use glam::IVec2;
use grid::Grid;

advent_of_code::solution!(12);

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
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(140))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(772))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 3), Some(1930))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), None)]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

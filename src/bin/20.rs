use advent_of_code::AocGrid;
use glam::IVec2;
use grid::Grid;
use pathfinding::prelude::dijkstra;

advent_of_code::solution!(20);

fn successors(pos: &IVec2, grid: &Grid<char>) -> Vec<(IVec2, usize)> {
    grid.neighbors(*pos)
        .filter(|(_, &n)| n != '#')
        .map(|(p, _)| (p, 1))
        .collect()
}

fn solve_part_one(input: &str, min_save: u64) -> Option<u32> {
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

    let (path, cost) = dijkstra(&start, |p| successors(p, &maze), |&p| p == finish).unwrap();

    let cheats = path.iter().enumerate().flat_map(|(start_cost, p)| {
        maze.neighbors(*p)
            .filter(|(_, &n)| n == '#')
            .map(|(wp, _)| {
                (
                    wp,
                    dijkstra(&wp, |nw| successors(nw, &maze), |&p| p == finish)
                        .unwrap()
                        .1,
                )
            })
            .filter_map(move |(wp, finish_cost)| {
                let save = cost as i64 - (start_cost + finish_cost) as i64 - 1;
                if save >= min_save as i64 {
                    Some((wp, save))
                } else {
                    None
                }
            })
    });
    Some(cheats.count() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve_part_one(input, 100)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        let result = solve_part_one(input, 20);
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

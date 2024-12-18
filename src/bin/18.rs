use advent_of_code::AocGrid;
use glam::IVec2;
use grid::Grid;
use pathfinding::prelude::dijkstra;

advent_of_code::solution!(18);

fn parse_line(line: &str) -> IVec2 {
    let coords = line.split(",").collect::<Vec<_>>();
    let (x, y) = (
        coords[0].parse::<usize>().unwrap(),
        coords[1].parse::<usize>().unwrap(),
    );
    IVec2::new(x as i32, y as i32)
}

fn solve_part_one(input: &str, size: usize, bytes: usize) -> Option<u32> {
    let mut input_vec = vec![1; size * size];

    input
        .lines()
        .by_ref()
        .take(bytes)
        .map(parse_line)
        .for_each(|byte| {
            input_vec[byte.y as usize * size + byte.x as usize] = 0;
        });

    let grid = Grid::from_vec(input_vec, size);

    let start = IVec2::new(0, 0);
    let finish = IVec2::new(size as i32 - 1, size as i32 - 1);

    let result: Option<(Vec<IVec2>, u32)> = dijkstra(
        &start,
        |&pos| {
            grid.neighbors(pos)
                .filter(|&(_, &v)| v == 1)
                .map(|(pos, _)| (pos, 1))
                .collect::<Vec<_>>()
        },
        |&pos| pos == finish,
    );

    Some(result?.1)
}

fn solve_part_two(input: &str, size: usize, bytes: usize) -> Option<String> {
    let mut input_vec = vec![1; size * size];
    let mut lines = input.lines();

    lines.by_ref().take(bytes).map(parse_line).for_each(|byte| {
        input_vec[byte.y as usize * size + byte.x as usize] = 0;
    });

    let mut grid = Grid::from_vec(input_vec, size);

    let start = IVec2::new(0, 0);
    let finish = IVec2::new(size as i32 - 1, size as i32 - 1);

    let mut testbyte = parse_line(lines.next().unwrap());

    while dijkstra(
        &start,
        |&pos| {
            grid.neighbors(pos)
                .filter(|&(_, &v)| v == 1)
                .map(|(pos, _)| (pos, 1))
                .collect::<Vec<_>>()
        },
        |&pos| pos == finish,
    )
    .is_some()
    {
        testbyte = parse_line(lines.next().unwrap());
        *grid.get_mut(testbyte.y, testbyte.x).unwrap() = 0;
    }

    Some(format!("{},{}", testbyte.x, testbyte.y))
}

pub fn part_one(input: &str) -> Option<u32> {
    solve_part_one(input, 71, 1024)
}

pub fn part_two(input: &str) -> Option<String> {
    solve_part_two(input, 71, 1024)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(22))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = solve_part_one(input, 7, 12);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(String::from("6,1")))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<String>) {
        tracing_init(Level::INFO);
        let result = solve_part_two(input, 7, 12);
        assert_eq!(result, expected);
    }
}

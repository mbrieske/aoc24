use glam::IVec2;
use grid::Grid;
use std::collections::HashSet;

advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileP1 {
    Empty,
    Wall,
    Box,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileP2 {
    Empty,
    Wall,
    BoxL,
    BoxR,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!("Unknown direction: {}", c),
        }
    }
}

impl TileP1 {
    fn from_char(c: char) -> Self {
        match c {
            '.' | '@' => Self::Empty,
            '#' => Self::Wall,
            'O' => Self::Box,
            _ => unreachable!("Unknown tile: {}", c),
        }
    }
}

impl TileP2 {
    fn from_char(c: char) -> Self {
        match c {
            '.' | '@' => Self::Empty,
            '#' => Self::Wall,
            '[' => Self::BoxL,
            ']' => Self::BoxR,
            _ => unreachable!("Unknown tile: {}", c),
        }
    }
}

fn display_p1(warehouse: &Grid<TileP1>, pos: &IVec2) {
    for (y, row) in warehouse.iter_rows().enumerate() {
        for (x, tile) in row.enumerate() {
            let c = if *pos == IVec2::new(x as i32, y as i32) {
                '@'
            } else {
                match tile {
                    TileP1::Empty => '.',
                    TileP1::Wall => '#',
                    TileP1::Box => 'O',
                }
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn display_p2(warehouse: &Grid<TileP2>, pos: &IVec2) {
    for (y, row) in warehouse.iter_rows().enumerate() {
        for (x, tile) in row.enumerate() {
            let c = if *pos == IVec2::new(x as i32, y as i32) {
                '@'
            } else {
                match tile {
                    TileP2::Empty => '.',
                    TileP2::Wall => '#',
                    TileP2::BoxL => '[',
                    TileP2::BoxR => ']',
                }
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

pub fn part_one(input: &str) -> Option<u32> {
    let splits = input.split("\n\n").collect::<Vec<_>>();
    let (warehouse, inputs) = match &splits[..] {
        &[warehouse, inputs, ..] => (warehouse, inputs),
        _ => unreachable!(),
    };

    let mut pos = IVec2::ZERO;

    let width = warehouse.lines().next().map(|l| l.chars().count()).unwrap();
    let mut warehouse = Grid::<TileP1>::from_vec(
        warehouse
            .chars()
            .filter(|c| c != &'\n')
            .enumerate()
            .inspect(|(i, c)| {
                if c == &'@' {
                    pos = IVec2::new((i % width) as i32, (i / width) as i32);
                }
            })
            .map(|(_, c)| TileP1::from_char(c))
            .collect(),
        width,
    );

    if tracing::enabled!(tracing::Level::DEBUG) {
        println!("Initial state:");
        display_p1(&warehouse, &pos);
    }

    inputs
        .chars()
        .filter(|c| c != &'\n')
        .map(Direction::from_char)
        .for_each(|c| {
            if tracing::enabled!(tracing::Level::DEBUG) {
                println!("Move: {:?}", c);
            }
            let dir = match c {
                Direction::Up => IVec2::NEG_Y,
                Direction::Down => IVec2::Y,
                Direction::Left => IVec2::NEG_X,
                Direction::Right => IVec2::X,
            };
            let mut next = pos + dir;
            let mut next_tile = warehouse.get(next.y, next.x).unwrap();
            let mut move_boxes: Vec<IVec2> = Vec::new();
            while next_tile == &TileP1::Box {
                move_boxes.push(next);
                next += dir;
                next_tile = warehouse.get(next.y, next.x).unwrap();
            }
            if next_tile == &TileP1::Empty {
                pos += dir;
                *warehouse.get_mut(pos.y, pos.x).unwrap() = TileP1::Empty;
                move_boxes.iter().for_each(|b| {
                    *warehouse.get_mut(b.y + dir.y, b.x + dir.x).unwrap() = TileP1::Box;
                });
            }
            if tracing::enabled!(tracing::Level::DEBUG) {
                display_p1(&warehouse, &pos);
            }
        });

    let res = warehouse
        .indexed_iter()
        .filter(|(_, &tile)| tile == TileP1::Box)
        .fold(0, |acc, ((y, x), _)| acc + (100 * y + x) as u32);
    Some(res)
}

fn can_move_updown(warehouse: &Grid<TileP2>, pos: IVec2, dir: IVec2) -> (bool, HashSet<IVec2>) {
    let tile = warehouse.get(pos.y, pos.x).unwrap();
    match tile {
        TileP2::Empty => (true, HashSet::new()),
        TileP2::Wall => (false, HashSet::new()),
        TileP2::BoxL => {
            let (can_move, mut move_boxes) = can_move_updown(warehouse, pos + dir, dir);
            let (can_move2, move_boxes2) = can_move_updown(warehouse, pos + dir + IVec2::X, dir);
            if can_move && can_move2 {
                move_boxes.extend(move_boxes2);
                move_boxes.insert(pos);
                (true, move_boxes)
            } else {
                (false, HashSet::new())
            }
        }
        TileP2::BoxR => {
            let (can_move, mut move_boxes) = can_move_updown(warehouse, pos + dir, dir);
            let (can_move2, move_boxes2) = can_move_updown(warehouse, pos + dir - IVec2::X, dir);
            if can_move && can_move2 {
                move_boxes.extend(move_boxes2);
                move_boxes.insert(pos - IVec2::X);
                (true, move_boxes)
            } else {
                (false, HashSet::new())
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let splits = input.split("\n\n").collect::<Vec<_>>();
    let (warehouse, inputs) = match &splits[..] {
        &[warehouse, inputs, ..] => (warehouse, inputs),
        _ => unreachable!(),
    };

    let mut pos = IVec2::ZERO;

    let width = warehouse.lines().next().map(|l| l.chars().count()).unwrap() * 2;
    let mut warehouse = Grid::<TileP2>::from_vec(
        warehouse
            .chars()
            .filter(|c| c != &'\n')
            .flat_map(|c| match c {
                '#' => vec!['#', '#'],
                'O' => vec!['[', ']'],
                '.' => vec!['.', '.'],
                '@' => vec!['@', '.'],
                _ => unreachable!("Unknown tile: {}", c),
            })
            .enumerate()
            .inspect(|(i, c)| {
                if c == &'@' {
                    pos = IVec2::new((i % width) as i32, (i / width) as i32);
                }
            })
            .map(|(_, c)| TileP2::from_char(c))
            .collect(),
        width,
    );

    if tracing::enabled!(tracing::Level::DEBUG) {
        println!("Initial state:");
        display_p2(&warehouse, &pos);
    }

    inputs
        .chars()
        .filter(|c| c != &'\n')
        .map(Direction::from_char)
        .for_each(|c| {
            if tracing::enabled!(tracing::Level::DEBUG) {
                println!("Move: {:?}", c);
            }
            let dir = match c {
                Direction::Up => IVec2::NEG_Y,
                Direction::Down => IVec2::Y,
                Direction::Left => IVec2::NEG_X,
                Direction::Right => IVec2::X,
            };

            if c == Direction::Left || c == Direction::Right {
                let mut next = pos + dir;
                let mut next_tile = warehouse.get(next.y, next.x).unwrap();
                let mut move_boxes: Vec<(IVec2, TileP2)> = Vec::new();
                while next_tile == &TileP2::BoxL || next_tile == &TileP2::BoxR {
                    move_boxes.push((next, *next_tile));
                    next += dir;
                    next_tile = warehouse.get(next.y, next.x).unwrap();
                }
                if next_tile == &TileP2::Empty {
                    pos += dir;
                    *warehouse.get_mut(pos.y, pos.x).unwrap() = TileP2::Empty;
                    move_boxes.iter().for_each(|(pos, boxtype)| {
                        *warehouse.get_mut(pos.y + dir.y, pos.x + dir.x).unwrap() = *boxtype;
                    });
                }
            } else {
                let (can_move, move_boxes) = can_move_updown(&warehouse, pos + dir, dir);
                if can_move {
                    pos += dir;
                    // sort move_boxes by y: if moving up, sort by y asc, if moving down, sort by y desc
                    let mut move_boxes = move_boxes.into_iter().collect::<Vec<_>>();
                    move_boxes.sort_by(|a, b| {
                        if dir.y < 0 {
                            a.y.cmp(&b.y)
                        } else {
                            b.y.cmp(&a.y)
                        }
                    });

                    move_boxes.iter().for_each(|b| {
                        // move_boxes only contain BoxL, so both b and b + IVec2::X need to be moved
                        *warehouse.get_mut(b.y, b.x).unwrap() = TileP2::Empty;
                        *warehouse.get_mut(b.y, b.x + 1).unwrap() = TileP2::Empty;
                        *warehouse.get_mut(b.y + dir.y, b.x).unwrap() = TileP2::BoxL;
                        *warehouse.get_mut(b.y + dir.y, b.x + 1).unwrap() = TileP2::BoxR;
                    });
                }
            }
            if tracing::enabled!(tracing::Level::DEBUG) {
                display_p2(&warehouse, &pos);
            }
        });

    let res = warehouse
        .indexed_iter()
        .filter(|(_, &tile)| tile == TileP2::BoxL)
        .fold(0, |acc, ((y, x), _)| acc + (100 * y + x) as u32);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(10092))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(2028))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(9021))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 3), None)]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

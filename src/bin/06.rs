use advent_of_code::AocGrid;
use grid::Grid;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_dir(&self, dir: &Direction) -> Option<Self> {
        match dir {
            Direction::Up => Some(Position {
                x: self.x,
                y: self.y.checked_sub(1)?,
            }),
            Direction::Down => Some(Position {
                x: self.x,
                y: self.y + 1,
            }),
            Direction::Left => Some(Position {
                x: self.x.checked_sub(1)?,
                y: self.y,
            }),
            Direction::Right => Some(Position {
                x: self.x + 1,
                y: self.y,
            }),
        }
    }
}

fn parse_input(input: &str) -> (Position, Grid<char>) {
    let mut index_start = 0;
    let grid = Grid::from_vec(
        input
            .chars()
            .filter(|&c| c != '\n')
            .enumerate()
            .inspect(|&(i, c)| {
                if c == '^' {
                    index_start = i;
                }
            })
            .map(|(_, c)| c)
            .collect::<Vec<char>>(),
        input.lines().next().unwrap().len(),
    );
    (
        Position {
            x: index_start % grid.size().0,
            y: index_start / grid.size().0,
        },
        grid,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut pos, mut grid) = parse_input(input);
    let mut dir = Direction::Up;

    *grid.get_mut(pos.y, pos.x).unwrap() = 'X';

    while let Some(next) = pos.move_dir(&dir) {
        match grid.get_mut(next.y, next.x) {
            Some('#') => {
                dir = dir.turn_right();
            }
            Some(field) => {
                pos = next;
                *field = 'X';
            }
            None => {
                break;
            }
        }
    }
    Some(grid.iter().filter(|&&c| c == 'X').count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start_pos, mut grid) = parse_input(input);
    let mut dir = Direction::Up;
    let mut possible_boulder_positions = HashMap::new();

    let mut pos = start_pos;
    while let Some(next) = pos.move_dir(&dir) {
        match grid.get(next.y, next.x) {
            Some('#') => {
                dir = dir.turn_right();
            }
            Some(_) => {
                pos = next;
                possible_boulder_positions.entry(pos).or_insert(dir);
            }
            None => {
                break;
            }
        }
    }
    possible_boulder_positions.remove(&start_pos);

    let loops = possible_boulder_positions
        .iter()
        .map(|boulder_pos| {
            *grid.get_mut(boulder_pos.0.y, boulder_pos.0.x).unwrap() = '#';

            let mut hashed_moves: HashMap<Position, HashSet<Direction>> = HashMap::new();
            dir = *boulder_pos.1;
            pos = match boulder_pos.1 {
                Direction::Up => Position {
                    x: boulder_pos.0.x,
                    y: boulder_pos.0.y + 1,
                },
                Direction::Down => Position {
                    x: boulder_pos.0.x,
                    y: boulder_pos.0.y - 1,
                },
                Direction::Left => Position {
                    x: boulder_pos.0.x + 1,
                    y: boulder_pos.0.y,
                },
                Direction::Right => Position {
                    x: boulder_pos.0.x - 1,
                    y: boulder_pos.0.y,
                },
            };

            let mut loop_found = false;
            while let Some(next) = pos.move_dir(&dir) {
                loop_found = !hashed_moves.entry(pos).or_default().insert(dir);
                if loop_found {
                    break;
                }
                match grid.get(next.y, next.x) {
                    Some('#') => {
                        dir = dir.turn_right();
                    }
                    Some(_) => {
                        pos = next;
                    }
                    None => {
                        break;
                    }
                }
            }
            *grid.get_mut(boulder_pos.0.y, boulder_pos.0.x).unwrap() = '.';
            loop_found
        })
        .filter(|&loop_found| loop_found)
        .count() as u32;
    Some(loops)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(41))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(6))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

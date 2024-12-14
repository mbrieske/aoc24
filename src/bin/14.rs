advent_of_code::solution!(14);

use glam::I64Vec2;
use nom::{
    bytes::complete::tag,
    character::complete,
    multi::many1,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crossterm::{
    cursor::MoveToColumn,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
};
use std::{
    io::{self, Write},
    time::Duration,
};

#[derive(Debug)]
struct Robot {
    pos: I64Vec2,
    v: I64Vec2,
}

impl Robot {
    fn step(&mut self, t: i64, w: i64, h: i64) {
        self.pos += self.v * t;
        self.pos.x = self.pos.x.rem_euclid(w);
        self.pos.y = self.pos.y.rem_euclid(h);
    }
}

fn robot(input: &str) -> IResult<&str, Robot> {
    let (input, pos) = preceded(
        tag("p="),
        separated_pair(complete::i64, tag(","), complete::i64),
    )(input)?;
    let (input, v) = delimited(
        tag(" v="),
        separated_pair(complete::i64, tag(","), complete::i64),
        tag("\n"),
    )(input)?;
    Ok((
        input,
        Robot {
            pos: I64Vec2::new(pos.0, pos.1),
            v: I64Vec2::new(v.0, v.1),
        },
    ))
}

fn parse(input: &str) -> Vec<Robot> {
    many1(robot)(input).unwrap().1.into_iter().collect()
}

fn solve_part_one(input: &str, t: i64, w: i64, h: i64) -> Option<u64> {
    let mut robots = parse(input);
    for robot in robots.iter_mut() {
        robot.step(t, w, h);
    }
    let res: u64 = robots
        .iter()
        .filter(|robot| robot.pos.x != w / 2 && robot.pos.y != h / 2)
        .fold(vec![0; 4], |mut acc, robot| {
            let mut quadrant = if robot.pos.x < w / 2 { 0 } else { 1 };
            quadrant += if robot.pos.y < h / 2 { 0 } else { 2 };
            acc[quadrant] += 1;
            acc
        })
        .iter()
        .fold(1, |mut acc, quadrant| {
            acc *= quadrant;
            acc
        });
    Some(res)
}

fn display_robots(robots: &[Robot], w: i64, h: i64, counter: u64) {
    let mut stdout = io::stdout();
    let mut grid = vec![vec![' '; w as usize]; h as usize];
    for robot in robots.iter() {
        grid[robot.pos.y as usize][robot.pos.x as usize] = '#';
    }
    for row in grid.iter() {
        execute!(stdout, MoveToColumn(0), Clear(ClearType::CurrentLine)).unwrap();
        println!("{}", row.iter().collect::<String>());
        stdout.flush().unwrap();
    }
    execute!(stdout, MoveToColumn(0), Clear(ClearType::CurrentLine)).unwrap();
    println!("Counter: {}\n", counter);
    stdout.flush().unwrap();
}

pub fn part_one(input: &str) -> Option<u64> {
    solve_part_one(input, 100, 101, 103)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut robots = parse(input);
    let w = 101;
    let h = 103;
    enable_raw_mode().unwrap();
    display_robots(&robots, w, h, 0);

    let mut counter = 0;

    loop {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(' ') => {
                        // solution is 7623
                        while counter < 7623 {
                            for robot in robots.iter_mut() {
                                robot.step(1, w, h);
                            }
                            counter += 1;
                        }
                        display_robots(&robots, w, h, counter);
                    }
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    // Disable raw mode before exiting
    disable_raw_mode().unwrap();
    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[test]
    fn test_robot_go() {
        let mut robot = Robot {
            pos: I64Vec2::new(0, 0),
            v: I64Vec2::new(1, 0),
        };
        robot.step(1, 10, 10);
        assert_eq!(robot.pos, I64Vec2::new(1, 0));
    }

    #[test]
    fn test_robot_wrap() {
        let mut robot = Robot {
            pos: I64Vec2::new(9, 9),
            v: I64Vec2::new(1, 1),
        };
        robot.step(1, 10, 10);
        assert_eq!(robot.pos, I64Vec2::new(0, 0));
    }

    #[test]
    fn test_robot_wrap_neg() {
        let mut robot = Robot {
            pos: I64Vec2::new(0, 0),
            v: I64Vec2::new(-1, -1),
        };
        robot.step(1, 10, 10);
        assert_eq!(robot.pos, I64Vec2::new(9, 9));
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(12))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u64>) {
        tracing_init(Level::INFO);
        let result = solve_part_one(input, 100, 101, 103);
        assert_eq!(result, expected);
    }
}

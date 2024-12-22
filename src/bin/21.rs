use std::hash::Hash;

use advent_of_code::AocGrid;
use cached::proc_macro::cached;
use glam::IVec2;
use grid::Grid;
use pathfinding::prelude::dijkstra;

advent_of_code::solution!(21);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PadType {
    NumberPad,
    Keypad,
}

impl PadType {
    fn to_string(&self) -> String {
        match self {
            PadType::NumberPad => String::from("NbrPad"),
            PadType::Keypad => String::from("KeyPad"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: IVec2,
    dir: IVec2,
}

impl State {
    fn new(init_pos: IVec2) -> Self {
        Self {
            pos: init_pos,
            dir: IVec2::ZERO,
        }
    }
}

fn numberpad_generate() -> Grid<char> {
    Grid::<char>::from_input("789\n456\n123\nX0A\n")
}

fn keypad_generate() -> Grid<char> {
    Grid::<char>::from_input("X^A\n<v>\n")
}

fn successors(state: &State, pad: &Grid<char>) -> Vec<(State, usize)> {
    pad.neighbors(state.pos)
        .filter(|(_, &nchar)| nchar != 'X')
        .map(|(npos, _)| {
            let ndir = npos - state.pos;
            let mut cost = 1;

            if ndir != state.dir && state.dir != IVec2::ZERO {
                cost += 1000;
            }

            (
                State {
                    pos: npos,
                    dir: ndir,
                },
                cost,
            )
        })
        .collect()
}

#[cached(
    key = "String",
    convert = r#"{ pad_type.to_string() + &c_from.to_string() + &c_to.to_string() }"#
)]
fn get_path(pad: &Grid<char>, pad_type: PadType, c_from: &char, c_to: &char) -> Vec<State> {
    dijkstra(
        &State::new(match_pad_pos(c_from, pad_type)),
        |state| successors(state, &pad),
        |state| pad.get_ivec(state.pos).unwrap() == c_to,
    )
    .unwrap()
    .0
}

fn match_dir(dir: IVec2) -> Option<char> {
    match dir {
        IVec2::X => Some('>'),
        IVec2::Y => Some('v'),
        IVec2::NEG_X => Some('<'),
        IVec2::NEG_Y => Some('^'),
        _ => Some('A'),
    }
}

fn match_pad_pos(btn: &char, pad_type: PadType) -> IVec2 {
    match pad_type {
        PadType::NumberPad => match_numberpad_pos(*btn),
        PadType::Keypad => match_keypad_pos(*btn),
    }
}

fn match_numberpad_pos(btn: char) -> IVec2 {
    let (x, y) = match btn {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!(),
    };
    IVec2::new(x, y)
}

fn match_keypad_pos(btn: char) -> IVec2 {
    let (x, y) = match btn {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!(),
    };
    IVec2::new(x, y)
}

fn translate_controls(pad: &Grid<char>, pad_type: PadType, input: &mut Vec<char>) -> Vec<char> {
    input.insert(0, 'A');

    let mut path = input
        .iter()
        .zip(input.iter().skip(1))
        .flat_map(|(c_from, c_to)| get_path(pad, pad_type, c_from, c_to))
        .collect::<Vec<_>>();

    let a = path.remove(0);
    path.push(a);

    path.iter()
        .flat_map(|state| match_dir(state.dir))
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let codes = input.lines().collect::<Vec<_>>();
    let checknums = codes
        .iter()
        .map(|code| code[..code.len() - 1].parse::<usize>().unwrap());

    let numberpad = numberpad_generate();
    let keypad = keypad_generate();

    let res = codes
        .iter()
        .zip(checknums)
        .map(|(code, checknum)| {
            let mut code = code.chars().collect::<Vec<_>>();
            let mut inputs = translate_controls(&numberpad, PadType::NumberPad, &mut code);
            for _ in 0..2 {
                inputs = translate_controls(&keypad, PadType::Keypad, &mut inputs);
            }
            let len = inputs.len() as usize;
            len * checknum
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let codes = input.lines().collect::<Vec<_>>();
    let checknums = codes
        .iter()
        .map(|code| code[..code.len() - 1].parse::<usize>().unwrap());

    let numberpad = numberpad_generate();
    let keypad = keypad_generate();

    let res = codes
        .iter()
        .zip(checknums)
        .map(|(code, checknum)| {
            let mut code = code.chars().collect::<Vec<_>>();
            let mut inputs = translate_controls(&numberpad, PadType::NumberPad, &mut code);
            dbg!(inputs.len() as usize * checknum);
            for _ in 0..5 {
                inputs = translate_controls(&keypad, PadType::Keypad, &mut inputs);
                dbg!(inputs.len() as usize * checknum);
            }
            let len = inputs.len() as usize;
            len * checknum
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

    #[test]
    fn test_numberpad_type() {
        let numberpad = numberpad_generate();
        dbg!(translate_controls(
            &numberpad,
            PadType::NumberPad,
            &mut "029A".chars().collect()
        )
        .iter()
        .collect::<String>());
    }

    #[test]
    fn test_keyberpad_type() {
        let keypad = keypad_generate();
        dbg!(
            translate_controls(&keypad, PadType::Keypad, &mut "<^>vA".chars().collect())
                .iter()
                .collect::<String>()
        );
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(126384))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<usize>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), None)]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<usize>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

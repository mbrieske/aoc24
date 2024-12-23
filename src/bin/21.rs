use advent_of_code::AocGrid;
use cached::proc_macro::cached;
use glam::IVec2;
use grid::Grid;
use once_cell::sync::Lazy;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;
use std::hash::Hash;

advent_of_code::solution!(21);

static NUMBERPAD: Lazy<Grid<char>> = Lazy::new(|| Grid::<char>::from_input("789\n456\n123\nX0A\n"));
static KEYPAD: Lazy<Grid<char>> = Lazy::new(|| Grid::<char>::from_input("X^A\n<v>\n"));

static NUMBERPAD_PATHS: Lazy<HashMap<(char, char), Vec<char>>> =
    Lazy::new(|| precompute_paths(&NUMBERPAD, PadType::NumberPad));

static KEYPAD_PATHS: Lazy<HashMap<(char, char), Vec<char>>> =
    Lazy::new(|| precompute_paths(&KEYPAD, PadType::Keypad));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PadType {
    NumberPad,
    Keypad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: IVec2,
    dir: IVec2,
    step: usize,
}

impl State {
    fn new(init_pos: IVec2) -> Self {
        Self {
            pos: init_pos,
            dir: IVec2::ZERO,
            step: 1,
        }
    }
}

fn successors(state: &State, pad: &Grid<char>) -> Vec<(State, usize)> {
    pad.neighbors(state.pos)
        .filter(|(_, &nchar)| nchar != 'X')
        .map(|(npos, _)| {
            let ndir = npos - state.pos;
            let mut cost = match ndir {
                IVec2::NEG_X => 10,
                IVec2::NEG_Y => 20,
                IVec2::Y => 30,
                IVec2::X => 40,
                _ => unreachable!(),
            };
            cost /= state.step;

            if ndir != state.dir && state.dir != IVec2::ZERO {
                cost += 1000;
            }

            (
                State {
                    pos: npos,
                    dir: ndir,
                    step: state.step + 1,
                },
                cost,
            )
        })
        .collect()
}

fn precompute_paths(pad: &Grid<char>, pad_type: PadType) -> HashMap<(char, char), Vec<char>> {
    let mut map = HashMap::new();
    let chars = pad
        .iter()
        .filter(|&&c| c != 'X')
        .copied()
        .collect::<Vec<_>>();
    for &c_from in &chars {
        for &c_to in &chars {
            let mut path = dijkstra(
                &State::new(match_pad_pos(&c_from, pad_type)),
                |state| successors(state, pad),
                |state| pad.get_ivec(state.pos).unwrap() == &c_to,
            )
            .unwrap()
            .0
            .iter()
            .map(|state| match_dir(state.dir).unwrap())
            .collect::<Vec<_>>();
            path.rotate_left(1);
            map.insert((c_from, c_to), path);
        }
    }
    map
}

fn get_path(pad_type: PadType, c_from: &char, c_to: &char) -> &'static Vec<char> {
    match pad_type {
        PadType::NumberPad => NUMBERPAD_PATHS.get(&(*c_from, *c_to)).unwrap(),
        PadType::Keypad => KEYPAD_PATHS.get(&(*c_from, *c_to)).unwrap(),
    }
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

fn translate_controls(pad_type: PadType, input: &[char]) -> Vec<char> {
    let mut new_input = Vec::with_capacity(input.len() + 1);
    new_input.push('A');
    new_input.extend_from_slice(input);

    let path = new_input
        .iter()
        .zip(new_input.iter().skip(1))
        .flat_map(|(c_from, c_to)| get_path(pad_type, c_from, c_to).to_owned())
        .collect::<Vec<_>>();
    path
}

#[cached]
fn expand_get_length(inputs: Vec<char>, iterations: usize) -> usize {
    if iterations == 0 {
        return inputs.len();
    } else {
        let expanded = translate_controls(PadType::Keypad, &inputs);
        let mut slices = Vec::new();
        let mut start = 0;
        (0..expanded.len()).for_each(|i| {
            if expanded[i] == 'A' {
                slices.push(expanded[start..=i].to_vec());
                start = i + 1;
            }
        });
        slices
            .into_iter()
            .map(|slice| expand_get_length(slice, iterations - 1))
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let codes = input.lines().collect::<Vec<_>>();
    let checknums: Vec<usize> = codes
        .iter()
        .map(|code| code[..code.len() - 1].parse::<usize>().unwrap())
        .collect();

    let res = codes
        .iter()
        .zip(checknums.iter())
        .map(|(code, &checknum)| {
            let code = code.chars().collect::<Vec<_>>();
            let mut inputs = translate_controls(PadType::NumberPad, &code);
            for _ in 0..2 {
                inputs = translate_controls(PadType::Keypad, &inputs);
            }
            let len = inputs.len();
            len * checknum
        })
        .sum::<usize>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let codes = input.lines().collect::<Vec<_>>();

    let checknums: Vec<usize> = codes
        .iter()
        .map(|code| code[..code.len() - 1].parse::<usize>().unwrap())
        .collect();

    let res = codes
        .iter()
        .zip(checknums.iter())
        .map(|(code, &checknum)| {
            let code = code.chars().collect::<Vec<_>>();
            let inputs = translate_controls(PadType::NumberPad, &code);

            let len = expand_get_length(inputs, 25);
            len * checknum
        })
        .sum::<usize>();

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
        let result = translate_controls(
            PadType::NumberPad,
            "029A".chars().collect::<Vec<_>>().as_ref(),
        );
        dbg!(result.iter().collect::<String>());
    }

    #[test]
    fn test_keyberpad_split() {
        let a = translate_controls(
            PadType::Keypad,
            &mut "<^>vA".chars().collect::<Vec<_>>().as_ref(),
        );
        let b = translate_controls(
            PadType::Keypad,
            &mut "<^>vA".chars().collect::<Vec<_>>().as_ref(),
        );
        let ab = a.into_iter().chain(b.into_iter()).collect::<Vec<_>>();
        let check_equal = translate_controls(
            PadType::Keypad,
            &mut "<^>vA<^>vA".chars().collect::<Vec<_>>().as_ref(),
        );
        assert_eq!(ab, check_equal)
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(126384))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<usize>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(154115708116294))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<usize>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

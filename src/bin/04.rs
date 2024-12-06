use ndarray::{s, Array2};
use regex::Regex;

advent_of_code::solution!(4);

fn transpose(m: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..m[0].len())
        .map(|i| m.iter().map(|r| r[i]).collect())
        .collect()
}

fn rotate_90(m: Vec<Vec<char>>) -> Vec<Vec<char>> {
    transpose(m)
        .into_iter()
        .map(|r| r.into_iter().rev().collect())
        .collect()
}

fn rotate_45(m: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if m.is_empty() || m[0].is_empty() {
        return vec![];
    }

    let mut r = Vec::new();

    let rows = m.len();
    let cols = m[0].len();

    for d in 0..(rows + cols - 1) {
        let mut diagonal = Vec::new();

        let start_row = if d < rows { d } else { rows - 1 };
        let end_row = if d >= cols { d - cols + 1 } else { 0 };

        for r in (end_row..=start_row).rev() {
            let c = d - r;
            diagonal.push(m[r][c]);
        }
        r.push(diagonal);
    }
    r
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let re = Regex::new(r"XMAS").unwrap();
    let mut findings = 0;

    findings += re.captures_iter(input).count() as u32;
    findings += re
        .captures_iter(&input.chars().rev().collect::<String>())
        .count() as u32;

    let transposed = transpose(puzzle.clone())
        .into_iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    findings += re.captures_iter(&transposed).count() as u32;
    findings += re
        .captures_iter(&transposed.chars().rev().collect::<String>())
        .count() as u32;

    let diagonalized_l = rotate_45(puzzle.clone())
        .into_iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    findings += re.captures_iter(&diagonalized_l).count() as u32;
    findings += re
        .captures_iter(&diagonalized_l.chars().rev().collect::<String>())
        .count() as u32;

    let diagonalized_r = rotate_45(rotate_90(puzzle))
        .into_iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    findings += re.captures_iter(&diagonalized_r).count() as u32;
    findings += re
        .captures_iter(&diagonalized_r.chars().rev().collect::<String>())
        .count() as u32;
    Some(findings)
}

pub fn part_two(input: &str) -> Option<u32> {
    let puzzle: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let a_positions: Vec<(usize, usize)> = puzzle
        .iter()
        .enumerate()
        .flat_map(|(ri, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == 'A')
                .map(|(ci, _)| (ri, ci))
                .collect::<Vec<_>>()
        })
        .collect();

    let valids = vec![
        Array2::from_shape_vec((3, 3), vec!['M', '.', 'M', '.', 'A', '.', 'S', '.', 'S']).unwrap(),
        Array2::from_shape_vec((3, 3), vec!['S', '.', 'S', '.', 'A', '.', 'M', '.', 'M']).unwrap(),
        Array2::from_shape_vec((3, 3), vec!['M', '.', 'S', '.', 'A', '.', 'M', '.', 'S']).unwrap(),
        Array2::from_shape_vec((3, 3), vec!['S', '.', 'M', '.', 'A', '.', 'S', '.', 'M']).unwrap(),
    ];

    let ndpuzzle = Array2::from_shape_vec(
        (puzzle.len(), puzzle.len()),
        puzzle.into_iter().flatten().collect(),
    )
    .unwrap();

    let xs = a_positions
        .into_iter()
        .filter(|&(r, c)| {
            r > 0 && r < (ndpuzzle.shape()[0] - 1) && c > 0 && c < (ndpuzzle.shape()[1] - 1)
        })
        .filter_map(|(r, c)| {
            let sub = ndpuzzle.slice(s![r - 1..=r + 1, c - 1..=c + 1]);
            let mut sub_mut = sub.to_owned();
            sub_mut[[0, 1]] = '.';
            sub_mut[[1, 0]] = '.';
            sub_mut[[1, 2]] = '.';
            sub_mut[[2, 1]] = '.';

            if valids.iter().any(|v| v == sub_mut) {
                Some((r, c))
            } else {
                None
            }
        });

    Some(xs.count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(18))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::DEBUG);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(9))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    fn transpose_test() {
        let m = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];
        let t = vec![
            vec!['a', 'd', 'g'],
            vec!['b', 'e', 'h'],
            vec!['c', 'f', 'i'],
        ];
        assert_eq!(transpose(m), t);
    }

    #[rstest]
    fn rotate_45_test() {
        let m = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];
        let r = vec![
            vec!['a'],
            vec!['d', 'b'],
            vec!['g', 'e', 'c'],
            vec!['h', 'f'],
            vec!['i'],
        ];
        assert_eq!(rotate_45(m), r);
    }

    #[rstest]
    fn rotate_90_test() {
        let m = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];
        let r = vec![
            vec!['g', 'd', 'a'],
            vec!['h', 'e', 'b'],
            vec!['i', 'f', 'c'],
        ];
        assert_eq!(rotate_90(m), r);
    }
}

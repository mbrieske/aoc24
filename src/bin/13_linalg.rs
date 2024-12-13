use advent_of_code::solution;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

solution!(13);

#[derive(Debug)]
struct Equation {
    ax: u64,
    ay: u64,
    bx: u64,
    by: u64,
    rx: u64,
    ry: u64,
}

fn equation(input: &str) -> IResult<&str, Equation> {
    let (input, (ax, ay)) = terminated(
        separated_pair(
            preceded(tag("Button A: X+"), complete::u64),
            tag(", Y+"),
            complete::u64,
        ),
        tag("\n"),
    )(input)?;
    let (input, (bx, by)) = terminated(
        separated_pair(
            preceded(tag("Button B: X+"), complete::u64),
            tag(", Y+"),
            complete::u64,
        ),
        tag("\n"),
    )(input)?;
    let (input, (rx, ry)) = terminated(
        separated_pair(
            preceded(tag("Prize: X="), complete::u64),
            tag(", Y="),
            complete::u64,
        ),
        alt((tag("\n\n"), tag("\n"))),
    )(input)?;
    Ok((
        input,
        Equation {
            ax,
            ay,
            bx,
            by,
            rx,
            ry,
        },
    ))
}

fn parse(input: &str) -> Vec<Equation> {
    many1(equation)(input).unwrap().1
}

fn solve_equation(eq: &Equation, offset: u64) -> u64 {
    let rx = eq.rx + offset;
    let ry = eq.ry + offset;

    let ax = eq.ax as i128;
    let ay = eq.ay as i128;
    let bx = eq.bx as i128;
    let by = eq.by as i128;
    let rx = rx as i128;
    let ry = ry as i128;

    let det = ax * by - ay * bx;
    if det == 0 {
        return 0;
    }

    // a = (rx*by - ry*bx) / det
    // b = (-rx*ay + ry*ax) / det

    let a_num = rx * by - ry * bx;
    let b_num = -rx * ay + ry * ax;

    if a_num % det != 0 || b_num % det != 0 {
        return 0;
    }

    let a = a_num / det;
    let b = b_num / det;

    3 * (a as u64) + (b as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse(input);
    let res: u64 = equations.into_iter().map(|eq| solve_equation(&eq, 0)).sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse(input);
    let res: u64 = equations
        .into_iter()
        .map(|eq| solve_equation(&eq, 10_000_000_000_000))
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(480))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u64>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }
}

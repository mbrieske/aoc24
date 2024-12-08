use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

advent_of_code::solution!(3);

#[derive(Debug, Clone)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, pair) = delimited(
        tag("mul("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn parse_part1(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, mul).map(|(_, ins)| ins))(input)
}

fn parse_part2(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_, ins)| ins))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse_part1(input).ok()?.1;
    Some(
        instructions
            .iter()
            .map(|ins| match ins {
                Instruction::Mul(a, b) => a * b,
                _ => panic!(),
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = parse_part2(input).ok()?.1;
    Some(
        instructions
            .iter()
            .fold((true, 0), |(do_mul, acc), ins| match (do_mul, ins) {
                (true, Instruction::Mul(a, b)) => (do_mul, acc + a * b),
                (false, Instruction::Mul(_, _)) => (do_mul, acc),
                (_, Instruction::Do) => (true, acc),
                (_, Instruction::Dont) => (false, acc),
            })
            .1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(161))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(48))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

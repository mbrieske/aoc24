use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use z3::ast::{Ast, Int};
use z3::{Config, Context, Optimize, SatResult};

advent_of_code::solution!(13);

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

pub fn part_one(input: &str) -> Option<u32> {
    let equations = parse(input);
    let cfg = Config::new();

    let res: u32 = equations
        .into_iter()
        .map(|eq| {
            let ctx = Context::new(&cfg);
            let opt = Optimize::new(&ctx);

            let a = Int::new_const(&ctx, "a");
            let b = Int::new_const(&ctx, "b");
            let upper_bound = Int::from_i64(&ctx, 100);

            let ax = Int::from_u64(&ctx, eq.ax);
            let ay = Int::from_u64(&ctx, eq.ay);

            let bx = Int::from_u64(&ctx, eq.bx);
            let by = Int::from_u64(&ctx, eq.by);

            let rx = Int::from_u64(&ctx, eq.rx);
            let ry = Int::from_u64(&ctx, eq.ry);

            let eq1 = (&a * &ax + &b * &bx)._eq(&rx);
            let eq2 = (&a * &ay + &b * &by)._eq(&ry);

            opt.assert(&eq1);
            opt.assert(&eq2);
            opt.assert(&a.le(&upper_bound));
            opt.assert(&b.le(&upper_bound));

            let three = Int::from_i64(&ctx, 3);
            let objective = &three * &a + &b;
            opt.minimize(&objective);

            match opt.check(&[]) {
                SatResult::Sat => {
                    let model = opt.get_model().unwrap();
                    let a_val = model.eval(&a, true).unwrap().as_i64().unwrap();
                    let b_val = model.eval(&b, true).unwrap().as_i64().unwrap();
                    a_val as u32 * 3 + b_val as u32
                }
                _ => 0,
            }
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse(input);
    let cfg = Config::new();

    let res: u64 = equations
        .into_iter()
        .map(|eq| {
            let ctx = Context::new(&cfg);
            let opt = Optimize::new(&ctx);

            let a = Int::new_const(&ctx, "a");
            let b = Int::new_const(&ctx, "b");

            let ax = Int::from_u64(&ctx, eq.ax);
            let ay = Int::from_u64(&ctx, eq.ay);

            let bx = Int::from_u64(&ctx, eq.bx);
            let by = Int::from_u64(&ctx, eq.by);

            let rx = Int::from_u64(&ctx, eq.rx + 10000000000000);
            let ry = Int::from_u64(&ctx, eq.ry + 10000000000000);

            let eq1 = (&a * &ax + &b * &bx)._eq(&rx);
            let eq2 = (&a * &ay + &b * &by)._eq(&ry);

            opt.assert(&eq1);
            opt.assert(&eq2);

            let three = Int::from_i64(&ctx, 3);
            let objective = &three * &a + &b;
            opt.minimize(&objective);

            match opt.check(&[]) {
                SatResult::Sat => {
                    let model = opt.get_model().unwrap();
                    let a_val = model.eval(&a, true).unwrap().as_i64().unwrap();
                    let b_val = model.eval(&b, true).unwrap().as_i64().unwrap();
                    a_val as u64 * 3 + b_val as u64
                }
                _ => 0,
            }
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

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), Some(480))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }
}

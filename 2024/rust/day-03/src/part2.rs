use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, instructions) = parse(input).map_err(|e| miette!("parse failed {}", e))?;

    let (_, result) = instructions
        .iter()
        .fold((ShouldProcess::Do, 0), |(process, acc), ins| match ins {
            Ins::Mul(a, b) => {
                if process == ShouldProcess::Do {
                    (process, acc + a * b)
                } else {
                    (process, acc)
                }
            }
            Ins::Do => (ShouldProcess::Do, acc),
            Ins::Dont => (ShouldProcess::Dont, acc),
        });

    Ok(result.to_string())
}

#[derive(PartialEq, Eq)]
enum ShouldProcess {
    Do,
    Dont,
}

#[derive(Debug, Clone)]
enum Ins {
    Mul(u32, u32),
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Ins> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Ins::Mul(pair.0, pair.1)))
}

fn instruction(input: &str) -> IResult<&str, Ins> {
    alt((
        value(Ins::Dont, tag("don't()")),
        value(Ins::Do, tag("do()")),
        mul,
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Ins>> {
    many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}

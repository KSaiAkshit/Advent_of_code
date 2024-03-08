use std::array;

use crate::custom_error::AocError;

use glam::{I64Vec3, Vec3Swizzles};
use ndarray::prelude::*;
use ndarray_linalg::{error::LinalgError, Solve};
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space0, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};
use nom_supreme::parser_ext::ArrayParser;

// Hailstone A: 19, 13, 30 @ -2, 1, -2
// Hailstone B: 20, 25, 34 @ -2, -2, -4
// Hailstones' paths will cross inside the test area (at x=11.667, y=16.667).

// Hailstone A: 19, 13, 30 @ -2, 1, -2
// Hailstone B: 12, 31, 28 @ -1, -2, -1
// Hailstones' paths will cross outside the test area (at x=6.2, y=19.4).

// Hailstone A: 19, 13, 30 @ -2, 1, -2
// Hailstone B: 20, 19, 15 @ 1, -5, -3
// Hailstones' paths crossed in the past for hailstone A.

// Hailstone A: 18, 19, 22 @ -1, -1, -2
// Hailstone B: 20, 25, 34 @ -2, -2, -4
// Hailstones' paths are parallel; they never intersect.

// Hailstone A: 18, 19, 22 @ -1, -1, -2
// Hailstone B: 12, 31, 28 @ -1, -2, -1
// Hailstones' paths will cross outside the test area (at x=-6, y=-5).

// Hailstone A: 18, 19, 22 @ -1, -1, -2
// Hailstone B: 20, 19, 15 @ 1, -5, -3
// Hailstones' paths crossed in the past for both hailstones.

// Hailstone A: 20, 25, 34 @ -2, -2, -4
// Hailstone B: 12, 31, 28 @ -1, -2, -1
// Hailstones' paths will cross outside the test area (at x=-2, y=3).

// Hailstone A: 20, 25, 34 @ -2, -2, -4
// Hailstone B: 20, 19, 15 @ 1, -5, -3
// Hailstones' paths crossed in the past for hailstone B.

// Hailstone A: 12, 31, 28 @ -1, -2, -1
// Hailstone B: 20, 19, 15 @ 1, -5, -3
// Hailstones' paths crossed in the past for both hailstones.
#[derive(Debug)]
struct Hail {
    starting_position: I64Vec3,
    direction: I64Vec3,
}

// 18, 19, 22 @
fn ivec(input: &str) -> IResult<&str, I64Vec3> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, z) = complete::i64(input)?;
    Ok((input, I64Vec3::new(x, y, z)))
}

fn parse_hails(input: &str) -> IResult<&str, Vec<Hail>> {
    separated_list1(
        line_ending,
        separated_pair(ivec, delimited(space1, tag("@"), space1), ivec).map(
            |(starting_position, direction)| Hail {
                starting_position,
                direction,
            },
        ),
    )(input)
}
// Hailstone A: 19, 13, 30 @ -2, 1, -2
// Hailstone B: 18, 19, 22 @ -1, -1, -2
// Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).

fn solve_matrix(h1: Hail, h2: Hail) {
    let coef_arr: Array2<f64> = array!(h1.direction.xy() as f64, h2.direction.xy() as f64);
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, hail) = parse_hails(input).expect("Should pass");
    dbg!(hail);
    Ok("result".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // bounds of [7, 27] (inclusive)
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}

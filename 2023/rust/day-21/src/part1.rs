use std::collections::HashSet;

use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::line_ending,
    combinator::opt,
    multi::fold_many1,
    sequence::terminated,
    IResult, Parser,
};
use nom_locate::LocatedSpan;

use crate::custom_error::AocError;

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

fn with_xy(span: Span) -> SpanIVec2 {
    // column/location are 1-indexed
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}

fn parse_grid(input: Span) -> IResult<Span, (HashSet<IVec2>, IVec2)> {
    fold_many1(
        terminated(
            alt((tag("."), tag("S"))).map(with_xy),
            opt(alt((line_ending, is_a("#")))),
        ),
        || (HashSet::new(), IVec2::splat(0)),
        |(mut set, mut start), next| {
            if next.fragment() == &"S" {
                start = next.extra;
            }
            set.insert(next.extra);
            (set, start)
        },
    )(input)
}

#[tracing::instrument]
pub fn process(input: &str, step_count: usize) -> miette::Result<String, AocError> {
    let (_input, (set, start)) = parse_grid(Span::new(input)).expect("a valid parse");
    let mut starting_hashset = HashSet::new();
    starting_hashset.insert(start);

    let last_set = std::iter::successors(Some(starting_hashset), |occupied_positions| {
        let mut new_set: HashSet<IVec2> = HashSet::new();

        for pos in occupied_positions.iter() {
            [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
                .into_iter()
                .filter_map(|offset| {
                    let cell = offset + *pos;
                    set.contains(&cell).then_some(cell)
                })
                .for_each(|pos| {
                    new_set.insert(pos);
                });
        }
        Some(new_set)
    })
    .nth(step_count)
    .unwrap();

    Ok(last_set.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!("16", process(input, 6)?);
        Ok(())
    }
}

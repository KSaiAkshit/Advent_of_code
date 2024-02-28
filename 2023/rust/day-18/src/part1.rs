

use crate::custom_error::AocError;
use glam::IVec2;
use itertools::{Itertools, MinMaxResult};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, hex_digit1, line_ending, space1},
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};
use tracing::{span, Level};

#[derive(Debug, Clone, Copy)]
struct DigInstructions<'a> {
    direction: IVec2,
    count: i32,
    color: &'a str,
}

fn parse_dig(input: &str) -> IResult<&str, DigInstructions> {
    let (input, direction) = alt((
        complete::char('R').map(|_| IVec2::X),
        complete::char('L').map(|_| IVec2::NEG_X),
        complete::char('U').map(|_| IVec2::Y),
        complete::char('D').map(|_| IVec2::NEG_Y),
    ))(input)?;
    let (input, count) = delimited(space1, complete::i32, space1)(input)?;
    let (input, hex) = delimited(tag("(#"), hex_digit1, complete::char(')'))(input)?;
    Ok((
        input,
        DigInstructions {
            direction,
            count,
            color: hex,
        },
    ))
}

fn instructions(input: &str) -> IResult<&str, Vec<DigInstructions>> {
    separated_list1(line_ending, parse_dig)(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, instructions) = instructions(input).unwrap();

    let travel: Vec<IVec2> = instructions
        .iter()
        .flat_map(|instruction| std::iter::repeat(instruction).take(instruction.count as usize))
        .scan(IVec2::new(0, 0), |acc, next| {
            *acc += next.direction;
            Some(*acc)
        })
        .collect();
    let MinMaxResult::MinMax(x_min, x_max) = travel.iter().map(|pos| pos.x).minmax() else {
        panic!("should have a min and max for x");
    };
    let MinMaxResult::MinMax(y_min, y_max) = travel.iter().map(|pos| pos.y).minmax() else {
        panic!("should have a min and max for y");
    };

    let interior_tiles = (y_min..=y_max)
        .map(|row| {
            ((x_min - 1)..x_max).fold(
                (0, None::<IVec2>, vec![]),
                |(mut crossings, mut last_crossing, mut all_interior_tiles), next_position| {
                    let next_ivec = IVec2::new(next_position, row);
                    let my_span = span!(
                        Level::INFO,
                        "row_span",
                        crossings,
                        ?last_crossing,
                        ?next_ivec,
                        row
                    );
                    my_span.in_scope(|| {
                        match travel.contains(&next_ivec) {
                            true => {
                                if last_crossing.is_none() {
                                    // info!("contains::is_none");
                                    crossings += 1;
                                    last_crossing = Some(next_ivec);
                                }
                            }
                            false => {
                                // info!("contains::false");
                                if last_crossing
                                    .is_some_and(|cross| (next_ivec - cross) == IVec2::new(1, 0))
                                {
                                    // info!("is_some_and");
                                    // if we land on an empty square
                                    // and the last crossing is directly
                                    // before the empty space, then reset
                                    // last_crossing
                                    last_crossing = None;
                                    if crossings % 2 == 0 {
                                        // outside
                                    } else {
                                        // inside
                                        all_interior_tiles.push(next_ivec);
                                    }
                                } else if last_crossing.is_some() {
                                    // info!("is_some");
                                    // if we land on an empty square
                                    // and the last crossing is *not* directly
                                    // before the empty space, then calculate if
                                    // we should cross
                                    let last_hash = next_ivec + IVec2::NEG_X;
                                    // info!(?last_hash);
                                    let last_hash_up = last_hash + IVec2::Y;
                                    let last_hash_down = last_hash + IVec2::NEG_Y;
                                    let last_hash_contains_up = travel.contains(&last_hash_up);
                                    let last_hash_contains_down = travel.contains(&last_hash_down);

                                    let last_cross = last_crossing.unwrap();
                                    let last_cross_up = last_cross + IVec2::Y;
                                    let last_cross_down = last_cross + IVec2::NEG_Y;
                                    let last_cross_contains_up = travel.contains(&last_cross_up);
                                    let last_cross_contains_down =
                                        travel.contains(&last_cross_down);
                                    // info!(
                                    //     last_hash_contains_up,
                                    //     last_cross_contains_up,
                                    //     last_hash_contains_down,
                                    //     last_cross_contains_down,
                                    // );
                                    if last_hash_contains_up && last_cross_contains_up
                                        || last_hash_contains_down && last_cross_contains_down
                                    {
                                        crossings += 1;
                                    }
                                    if crossings % 2 == 0 {
                                        // outside
                                    } else {
                                        // inside
                                        all_interior_tiles.push(next_ivec);
                                    }
                                    last_crossing = None;
                                } else {
                                    // info!("last_crossing::is_none");
                                    if crossings % 2 == 0 {
                                        // outside
                                    } else {
                                        // inside
                                        all_interior_tiles.push(next_ivec);
                                    }
                                }
                            }
                        }

                        (crossings, last_crossing, all_interior_tiles)
                    })
                },
            )
        })
        .flat_map(|x| x.2)
        .collect::<Vec<IVec2>>();

    // write_grid(&travel, y_min..=y_max, x_min..=x_max);

    let grid = travel
        .iter()
        .chain(interior_tiles.iter())
        .cloned()
        .collect::<Vec<IVec2>>();

    // write_grid(&grid, y_min..=y_max, x_min..=x_max);

    Ok(grid.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!("62", process(input)?);
        Ok(())
    }
}

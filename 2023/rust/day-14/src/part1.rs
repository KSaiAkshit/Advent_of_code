use crate::custom_error::AocError;

use glam::IVec2;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Rock {
    Movable,
    Immovable,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let columns = input.lines().next().unwrap().len();
    let rows = input.lines().count();
    let boundaries = IVec2::new(columns as i32, rows as i32);
    let rock_map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                'O' => Some((IVec2::new(x as i32, y as i32), Rock::Movable)),
                '#' => Some((IVec2::new(x as i32, y as i32), Rock::Immovable)),
                _ => None,
            })
        })
        .collect::<HashMap<IVec2, Rock>>();

    let static_rocks = rock_map
        .iter()
        .filter_map(|(position, rock)| match rock {
            Rock::Movable => None,
            Rock::Immovable => Some((*position, *rock)),
        })
        .collect::<HashMap<IVec2, Rock>>();
    let next_state = rock_shift_north(&rock_map, &boundaries, &static_rocks);

    let sum = next_state
        .iter()
        .filter_map(|(position, rock)| match rock {
            Rock::Movable => Some(boundaries.y - position.y),
            Rock::Immovable => None,
        })
        .sum::<i32>();

    Ok(sum.to_string())
}

fn rock_shift_north(
    rock_map: &HashMap<IVec2, Rock>,
    boundaries: &IVec2,
    static_rocks: &HashMap<IVec2, Rock>,
) -> HashMap<IVec2, Rock> {
    let mut results = static_rocks.clone();

    let mut next_potentially_available_position: IVec2;
    for x in 0..boundaries.x {
        next_potentially_available_position = IVec2::new(x, 0);
        for y in 0..boundaries.y {
            match rock_map.get(&IVec2::new(x, y)) {
                Some(Rock::Immovable) => {
                    next_potentially_available_position = IVec2::new(x, y + 1);
                }
                Some(Rock::Movable) => {
                    let next_pos = next_potentially_available_position;
                    results.insert(next_pos, Rock::Movable);

                    next_potentially_available_position.y += 1;
                }
                None => {}
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!("136", process(input)?);
        Ok(())
    }
}

use crate::custom_error::AocError;
use glam::IVec2;
use itertools::Itertools;
use tracing::{info, span, Level};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    Ok("result".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#..... ";
        assert_eq!("", process(input)?);
        Ok(())
    }
}

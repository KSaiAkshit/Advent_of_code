use lending_iterator::prelude::*;
use tracing::{debug, info};

use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .rev()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let start_numbers = std::iter::from_fn(move || {
                if nums.iter().all(|num| num == &0) {
                    None
                } else {
                    let mut it = nums.windows_mut();
                    while let Some(&mut [ref mut left, right]) = it.next() {
                        *left -= right;
                    }

                    nums.pop()
                }
            })
            .collect::<Vec<i64>>();

            debug!(?start_numbers);
            let result = start_numbers.iter().rev().fold(0, |acc, num| {
                info!(acc, num, result = num - acc);
                num - acc
            });

            result
        })
        .sum::<i64>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}

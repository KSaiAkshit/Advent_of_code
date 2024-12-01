use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (left, right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            (parts[0], parts[1])
        })
        .unzip();
    let counts = right.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    let score: i32 = left
        .iter()
        .map(|&num| num * counts.get(&num).unwrap_or(&0))
        .sum();
    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}

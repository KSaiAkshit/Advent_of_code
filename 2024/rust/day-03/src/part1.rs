use miette::{miette, Context, IntoDiagnostic};
use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .into_diagnostic()
        .context("regex init failed")?;

    let mut xy: Vec<(u32, u32)> = Vec::new();
    for (_, [f1, f2]) in reg.captures_iter(input).map(|caps| caps.extract()) {
        xy.push((
            f1.parse().map_err(|e| miette!("parsing f1 failed {}", e))?,
            f2.parse().map_err(|e| miette!("parsing f2 failed {}", e))?,
        ));
    }

    let mut result: u32 = 0;
    for (x, y) in &xy {
        result += x * y;
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%
        &mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}

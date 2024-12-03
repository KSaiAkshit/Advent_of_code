use miette::{miette, Context, IntoDiagnostic};
use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let reg = Regex::new(r"mul\((-?\d{1,3}),(-?\d{1,3})\)")
        .into_diagnostic()
        .context("regex init failed")?;

    let input = input
        .replace("do()", "mul(-1,1)")
        .replace("don't()", "mul(-1,2)");

    let mut xy: Vec<(i32, i32)> = Vec::new();
    for (_, [f1, f2]) in reg.captures_iter(&input).map(|caps| caps.extract()) {
        xy.push((
            f1.parse().map_err(|e| miette!("parsing f1 failed {}", e))?,
            f2.parse().map_err(|e| miette!("parsing f2 failed {}", e))?,
        ));
    }

    let mut result: Vec<i32> = Vec::default();
    for (x, y) in &xy {
        result.push(x * y);
    }
    let mut should_add = true;
    let mut total = 0;

    for num in result {
        match num {
            -1 => {
                should_add = true;
            }
            -2 => {
                should_add = false;
            }
            _ if should_add => {
                total += num;
            }
            _ => {}
        }
    }

    Ok(total.to_string())
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

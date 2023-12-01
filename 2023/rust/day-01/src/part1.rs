use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut result: Vec<(char, char)> = Vec::new();

    input.lines().for_each(|line| {
        let (first_digit, last_digit) =
            line.chars()
                .fold((None, None), |acc, c| match c.is_ascii_digit() {
                    true => {
                        let first_digit = acc.0.unwrap_or(c);
                        (Some(first_digit), Some(c))
                    }
                    false => {
                        let last_digit = acc.1.unwrap_or(c);
                        (acc.0, Some(last_digit))
                    }
                });

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            result.push((first, last));
        }
    });
    let digit_result: Vec<u32> = result
        .iter()
        .map(|&(a, b)| {
            let first = a.to_digit(10).unwrap();
            let second = b.to_digit(10).unwrap();
            first * 10 + second
        })
        .collect();

    dbg!(&digit_result.iter().sum::<u32>());
    let sum = digit_result.iter().sum::<u32>();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    input.lines().for_each(|line| {
        let nums = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<i32>>();
        reports.push(nums);
    });
    let mut result = 0;
    for report in reports {
        if is_safe(&report) || can_become_safe(&report) {
            result += 1;
        }
    }
    Ok(result.to_string())
}

fn is_safe(report: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..report.len() - 1 {
        let diff = report[i + 1] - report[i];
        if diff.abs() >= 4 || diff == 0 {
            return false;
        }
        if diff < 0 {
            increasing = false;
        } else {
            decreasing = false;
        }
    }

    increasing || decreasing
}

fn can_become_safe(report: &[i32]) -> bool {
    // Try removing each level and check if the modified report is safe
    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i);
        if is_safe(&modified_report) {
            return true; // Found a way to make it safe by removing one level
        }
    }
    false // No removal made it safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}

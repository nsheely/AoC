// src/bin/part1.rs

use day_02::part1::safe_reports;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = safe_reports(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_reports_example() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        let result = safe_reports(input);
        assert_eq!(result, 2); // Based on the example, 2 reports are safe
    }

    #[test]
    fn test_safe_reports_empty() {
        let input = "";
        let result = safe_reports(input);
        assert_eq!(result, 0); // No reports, so none can be safe
    }

    #[test]
    fn test_safe_reports_all_safe() {
        let input = "1 2 3 4 5\n10 9 8 7 6\n2 5 8 11 14";
        let result = safe_reports(input);
        assert_eq!(result, 3); // All reports are safe
    }

    #[test]
    fn test_safe_reports_all_unsafe() {
        let input = "1 2 3 7 8\n10 9 8 4 3\n2 5 8 12 14";
        let result = safe_reports(input);
        assert_eq!(result, 0); // All reports are unsafe
    }

    #[test]
    fn test_safe_reports_mixed() {
        let input = "1 2 3 4 5\n1 2 5 8 9\n9 7 4 3 1\n8 6 6 4 3";
        let result = safe_reports(input);
        assert_eq!(result, 3); // Fourth report is not safe
    }
}

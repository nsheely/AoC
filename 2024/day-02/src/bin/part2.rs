// src/bin/part2.rs

use day_02::part2::safe_reports;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = safe_reports(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_reports_with_dampener_example() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        let result = safe_reports(input);
        assert_eq!(result, 4); // Based on the example, 4 reports are safe with the dampener
    }

    #[test]
    fn test_safe_reports_with_dampener_empty() {
        let input = "";
        let result = safe_reports(input);
        assert_eq!(result, 0); // No reports, so none can be safe
    }

    #[test]
    fn test_safe_reports_with_dampener_all_safe() {
        let input = "1 2 3 4 5\n10 9 8 7 6\n2 5 8 11 14";
        let result = safe_reports(input);
        assert_eq!(result, 3); // All reports are safe even without the dampener
    }

    #[test]
    fn test_safe_reports_with_dampener_all_unsafe() {
        let input = "1 2 3 8 9\n10 9 8 4 3\n2 5 8 13 14";
        let result = safe_reports(input);
        assert_eq!(result, 0); // All reports are unsafe even with the dampener
    }

    #[test]
    fn test_safe_reports_with_dampener_mixed() {
        let input = "1 2 3 4 5\n1 2 5 8 9\n9 7 4 3 1\n8 6 6 4 3";
        let result = safe_reports(input);
        assert_eq!(result, 4); // With dampener, fourth report becomes safe
    }

    #[test]
    fn test_safe_reports_with_dampener_single_bad_level() {
        let input = "1 2 5 4 5\n5 4 3 1 0\n2 2 3 4 5";
        let result = safe_reports(input);
        assert_eq!(result, 3); // All reports are safe after removing one bad level
    }

    #[test]
    fn test_safe_reports_with_dampener_no_change_needed() {
        let input = "3 6 9 12 15\n15 12 9 6 3\n5 8 11 14 17";
        let result = safe_reports(input);
        assert_eq!(result, 3); // All reports are already safe
    }

    #[test]
    fn test_safe_reports_with_dampener_multiple_bad_levels() {
        let input = "1 2 6 7 8\n9 5 1 0 -2\n3 4 8 12 16";
        let result = safe_reports(input);
        assert_eq!(result, 0); // Reports cannot be made safe by removing just one level
    }
}

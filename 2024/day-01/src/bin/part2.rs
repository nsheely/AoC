// src/bin/part1.rs
use day_01::part2::similarity_score;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = similarity_score(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity_score_example() {
        let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
        let result = similarity_score(input);
        assert_eq!(result, 31); // 9 + 4 + 0 + 0 + 9 + 9
    }

    #[test]
    fn test_similarity_score_no_matches() {
        let input = "1 2\n3 4\n5 6";
        let result = similarity_score(input);
        assert_eq!(result, 0); // No numbers appear in both lists
    }

    #[test]
    fn test_similarity_score_all_same() {
        let input = "5 5\n5 5\n5 5";
        // Left has three 5s, right has three 5s
        // Each 5 in left contributes 5 * 3 = 15
        let result = similarity_score(input);
        assert_eq!(result, 45); // 15 + 15 + 15
    }

    #[test]
    fn test_similarity_score_multiple_matches() {
        let input = "1 2\n2 1\n1 1";
        // Left has two 1s and one 2
        // Right has two 1s and one 2
        // Each 1 in left contributes 1 * 2 = 2
        // The 2 in left contributes 2 * 1 = 2
        let result = similarity_score(input);
        assert_eq!(result, 6); // 2 + 2 + 2
    }

    #[test]
    fn test_similarity_score_empty_input() {
        let input = "";
        let result = similarity_score(input);
        assert_eq!(result, 0);
    }
}
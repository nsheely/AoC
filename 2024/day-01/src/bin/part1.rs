// src/bin/part1.rs

use day_01::part1::distance_between_lists;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = distance_between_lists(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_between_lists_example() {
        let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
        let result = distance_between_lists(input);
        assert_eq!(result, 11); // From the example provided in the problem description
    }

    #[test]
    fn test_distance_between_lists_multiple_lines() {
        let input = "3 5\n8 2\n10 10";
        // Left list: [3, 8, 10]
        // Right list: [5, 2, 10]
        // Sorted left: [3, 8, 10]
        // Sorted right: [2, 5, 10]
        // Distances: |3 - 2| = 1, |8 - 5| = 3, |10 - 10| = 0
        // Total distance: 1 + 3 + 0 = 4
        let result = distance_between_lists(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_distance_between_lists_all_same_values() {
        let input = "7 7\n7 7\n7 7";
        let result = distance_between_lists(input);
        assert_eq!(result, 0); // All distances are 0 since the values are the same
    }

    #[test]
    fn test_distance_between_lists_large_numbers() {
        let input = "100000 99999\n123456 123455\n987654 987653";
        let result = distance_between_lists(input);
        assert_eq!(result, 3); // Each pair differs by 1
    }

    #[test]
    fn test_distance_between_lists_negative_case() {
        let input = "10 2\n50 100\n75 25";
        // Left list: [10, 50, 75]
        // Right list: [2, 100, 25]
        // Sorted left: [10, 50, 75]
        // Sorted right: [2, 25, 100]
        // Distances: |10 - 2| = 8, |50 - 25| = 25, |75 - 100| = 25
        // Total distance: 8 + 25 + 25 = 58
        let result = distance_between_lists(input);
        assert_eq!(result, 58);
    }

    #[test]
    fn test_distance_between_lists_empty_input() {
        let input = "";
        let result = distance_between_lists(input);
        assert_eq!(result, 0); // Empty input should result in 0
    }

    #[test]
    fn test_distance_between_lists_large_input() {
        let input = (1..=1000)
            .map(|i| format!("{} {}", i, i))
            .collect::<Vec<_>>()
            .join("\n");
        let result = distance_between_lists(&input);
        assert_eq!(result, 0); // All pairs are identical
    }
}



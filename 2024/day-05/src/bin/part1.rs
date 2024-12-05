// src/bin/part1.rs

use day_05::part1::sum_middle_pages_of_correct_updates;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = sum_middle_pages_of_correct_updates(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let result = sum_middle_pages_of_correct_updates(input);
        assert_eq!(result, 143);
    }
}

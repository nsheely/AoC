// src/bin/part1.rs

use day_04::part1::count_xmas_occurrences;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = count_xmas_occurrences(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_grid() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let count = count_xmas_occurrences(input);
        assert_eq!(count, 18);
    }

    #[test]
    fn test_single_occurrence() {
        let input = "\
X M A S
A B C D
E F G H
I J K L";

        let input_cleaned: String = input
            .lines()
            .map(|line| line.chars().filter(|&c| c != ' ').collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        let count = count_xmas_occurrences(&input_cleaned);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_no_occurrence() {
        let input = "\
A B C D
E F G H
I J K L
M N O P";

        let input_cleaned: String = input
            .lines()
            .map(|line| line.chars().filter(|&c| c != ' ').collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        let count = count_xmas_occurrences(&input_cleaned);
        assert_eq!(count, 0);
    }
}


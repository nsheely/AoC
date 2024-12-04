// src/bin/part2.rs

use day_04::part2::count_xmas_occurrences;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = count_xmas_occurrences(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_grid_part2() {
        let input = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

        let count = count_xmas_occurrences(input);
        assert_eq!(count, 9);
    }

    #[test]
    fn test_single_xmas() {
        let input = "\
M . S
 . A .
M . S";

        let input_cleaned: String = input
            .lines()
            .map(|line| line.chars().filter(|&c| c != ' ').collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        let count = count_xmas_occurrences(&input_cleaned);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_no_xmas() {
        let input = "\
A B C
D E F
G H I";

        let input_cleaned: String = input
            .lines()
            .map(|line| line.chars().filter(|c| !c.is_whitespace()).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        let count = count_xmas_occurrences(&input_cleaned);
        assert_eq!(count, 0);
    }
}

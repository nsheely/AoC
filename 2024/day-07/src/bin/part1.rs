// src/bin/part1.rs

use day_07::part1::total_calibration_result;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = total_calibration_result(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(total_calibration_result(input), 3749);
    }
}

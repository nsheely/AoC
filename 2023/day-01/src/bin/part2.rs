// src/bin/part1.rs

use day_01::part2::sum_calibration_values;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = sum_calibration_values(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_calibration_values() {
        let document = "two1nine\n\
                        eightwothree\n\
                        abcone2threexyz\n\
                        xtwone3four\n\
                        4nineeightseven2\n\
                        zoneight234\n\
                        7pqrstsixteen";
        assert_eq!(sum_calibration_values(document), 281);
    }

    #[test]
    fn multiple_instances() {
        let document = "one2three4\nfive6seven";
        assert_eq!(sum_calibration_values(document), 71);
    }

    #[test]
    fn overlapping_spelled_numbers() {
        let document = "onetwothree\nfourfivesix";
        assert_eq!(sum_calibration_values(document), 59);
    }

    #[test]
    fn numbers_next_to_each_other() {
        let document = "2three\nfour5";
        assert_eq!(sum_calibration_values(document), 68);
    }

    #[test]
    fn no_valid_numbers() {
        let document = "abc\ndef";
        assert_eq!(sum_calibration_values(document), 0);
    }

    #[test]
    fn only_spelled_out_or_digits() {
        let document = "one\n2";
        assert_eq!(sum_calibration_values(document), 33);
    }

    #[test]
    fn long_lines_with_scattered_numbers() {
        let document = "a1b2c3d4e\nf5g6h7i8";
        assert_eq!(sum_calibration_values(document), 72);
    }

    #[test]
    fn eight_three() {
        let document = "eighthree";
        assert_eq!(sum_calibration_values(document), 83);
    }

    #[test]
    fn seven_nine() {
        let document = "sevenine";
        assert_eq!(sum_calibration_values(document), 79);
    }
}

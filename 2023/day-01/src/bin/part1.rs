// src/bin/part1.rs

use day_01::part1::sum_calibration_values;

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
        let document = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(sum_calibration_values(document), 142);
    }
}

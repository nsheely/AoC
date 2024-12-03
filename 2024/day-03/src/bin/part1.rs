// src/bin/part1.rs

use day_03::part1::sum_mul_results;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = sum_mul_results(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_mul_results_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = sum_mul_results(input);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_sum_mul_results_empty() {
        let input = "";
        let result = sum_mul_results(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_mul_results_no_valid_instructions() {
        let input = "invalid data without any mul instructions";
        let result = sum_mul_results(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_mul_results_single_instruction() {
        let input = "some text mul(12,34) more text";
        let result = sum_mul_results(input);
        assert_eq!(result, 12 * 34);
    }

    #[test]
    fn test_sum_mul_results_multiple_instructions() {
        let input = "mul(1,2)mul(3,4)mul(5,6)";
        let result = sum_mul_results(input);
        assert_eq!(result, 1*2 + 3*4 + 5*6);
    }

    #[test]
    fn test_sum_mul_results_invalid_instructions() {
        let input = "mul(1234,5) mul(6,7890) mul(12 34) mul(,)";
        let result = sum_mul_results(input);
        assert_eq!(result, 0);
    }
}

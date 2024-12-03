// src/bin/part2.rs

use day_03::part2::sum_mul_results;

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

    #[test]
    fn test_no_instructions() {
        let input = "some random text without instructions";
        let result = sum_mul_results(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_all_disabled() {
        let input = "don't()mul(10,10)mul(20,20)do_not_mul(30,30)";
        let result = sum_mul_results(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_enable_disable() {
        let input = "mul(1,1)don't()mul(2,2)do()mul(3,3)don't()mul(4,4)do()mul(5,5)";
        let result = sum_mul_results(input);
        // Enabled mul instructions: mul(1,1), mul(3,3), mul(5,5)
        assert_eq!(result, 1*1 + 3*3 + 5*5);
    }

    #[test]
    fn test_multiple_dos_and_donts() {
        let input = "do()mul(2,2)don't()don't()mul(3,3)do()do()mul(4,4)";
        let result = sum_mul_results(input);
        // Enabled mul instructions: mul(2,2), mul(4,4)
        assert_eq!(result, 2*2 + 4*4);
    }

}

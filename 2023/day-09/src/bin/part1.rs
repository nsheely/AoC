// src/bin/part1.rs


use day_09::part1::extrapolate_and_sum;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = extrapolate_and_sum(input);
    dbg!(output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "\
            0 3 6 9 12 15\n\
            1 3 6 10 15 21\n\
            10 13 16 21 30 45";
        assert_eq!(extrapolate_and_sum(input), 18 + 28 + 68);
    }

    #[test]
    fn empty_input() {
        let input = "";
        assert_eq!(extrapolate_and_sum(input), 0);
    }

    #[test]
    fn single_history() {
        let input = "1 2 3 4 5";
        assert_eq!(extrapolate_and_sum(input), 6);
    }

    #[test]
    fn all_zeroes() {
        let input = "0 0 0 0 0";
        assert_eq!(extrapolate_and_sum(input), 0);
    }

    #[test]
    fn negative_numbers() {
        let input = "-5 -3 -1 1 3 5";
        assert_eq!(extrapolate_and_sum(input), 7);
    }

    #[test]
    fn large_numbers() {
        let input = "1000000 2000000 3000000 4000000";
        assert_eq!(extrapolate_and_sum(input), 5000000);
    }

    #[test]
    fn mixed_sign_numbers() {
        let input = "-5 0 5 -10 10";
        assert_eq!(extrapolate_and_sum(input), 195);
    }

    #[test]
    fn zeros_mixed_with_negative_and_positive() {
        let input = "0 -5 0 5 0";
        assert_eq!(extrapolate_and_sum(input), -25);
    }

    #[test]
    fn only_negative_numbers() {
        let input = "-10 -20 -30 -40";
        assert_eq!(extrapolate_and_sum(input), -50);
    }

    #[test]
    fn single_number() {
        let input = "5";
        assert_eq!(extrapolate_and_sum(input), 5);
    }
}


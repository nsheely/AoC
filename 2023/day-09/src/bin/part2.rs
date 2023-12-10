// src/bin/part2.rs


use day_09::part2::extrapolate_and_sum;

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
        assert_eq!(extrapolate_and_sum(input), 2);
    }
}


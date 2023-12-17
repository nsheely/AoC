// src/bin/part1.rs

use day_15::part2::compute_focusing_power;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = compute_focusing_power(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(compute_focusing_power(input), 145);
    }
}

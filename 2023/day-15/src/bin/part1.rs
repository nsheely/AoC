// src/bin/part1.rs

use day_15::part1::compute_hash;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = compute_hash(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(compute_hash(input), 1320);
    }
}

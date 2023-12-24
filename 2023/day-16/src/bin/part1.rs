// src/bin/part1.rs

use day_16::part1::num_energized;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = num_energized(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input =  ".|...\\....
                            |.-.\\.....
                            .....|-...
                            ........|.
                            ..........
                            ..........
                            ..../.\\\\..
                            .-.-/..|..
                            .|....-|.\\
                            ..//.|....";
        assert_eq!(num_energized(input), 46);
    }
}

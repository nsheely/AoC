// src/bin/part1.rs

use day_16::part2::max_energized;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = max_energized(input);
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
        assert_eq!(max_energized(input), 51);
    }
}

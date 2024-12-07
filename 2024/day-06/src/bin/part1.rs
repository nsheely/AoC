// src/bin/part1.rs

use day_06::part1::guard_positions;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = guard_positions(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(guard_positions(input), 41);
    }
}

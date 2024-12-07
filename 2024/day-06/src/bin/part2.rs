// src/bin/part2.rs

use day_06::part2::count_guard_loop_additions;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = count_guard_loop_additions(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example() {
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
        assert_eq!(count_guard_loop_additions(input), 6);
    }
}

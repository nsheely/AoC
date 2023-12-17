// src/bin/part1.rs

use day_14::part1::tilt_and_sum_load;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = tilt_and_sum_load(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_image() {
        let input = "\
        O....#....\n\
        O.OO#....#\n\
        .....##...\n\
        OO.#O....O\n\
        .O.....O#.\n\
        O.#..O.#.#\n\
        ..O..#O..O\n\
        .......O..\n\
        #....###..\n\
        #OO..#....";
        assert_eq!(tilt_and_sum_load(input), 136);
    }
}

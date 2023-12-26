// src/bin/part1.rs

use day_18::part1::calculate_lava_capacity;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = calculate_lava_capacity(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lava_capacity_p1() {
        let dig_plan = "\
        R 6 (#70c710)\n\
        D 5 (#0dc571)\n\
        L 2 (#5713f0)\n\
        D 2 (#d2c081)\n\
        R 2 (#59c680)\n\
        D 2 (#411b91)\n\
        L 5 (#8ceee2)\n\
        U 2 (#caa173)\n\
        L 1 (#1b58a2)\n\
        U 2 (#caa171)\n\
        R 2 (#7807d2)\n\
        U 3 (#a77fa3)\n\
        L 2 (#015232)\n\
        U 2 (#7a21e3)";
        assert_eq!(calculate_lava_capacity(dig_plan), 62);
    }
}

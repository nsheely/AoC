// src/bin/part1.rs

// sum up all part numbers, a part number has an adjacent symbol ( not . ) in the list

use day_03::part2::sum_of_gear_ratios;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = sum_of_gear_ratios(input);
    dbg!(output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_engine_schematic() {
        let input = "\
        467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..";

        assert_eq!(sum_of_gear_ratios(input), 467835);
    }
}

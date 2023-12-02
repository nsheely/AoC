// src/bin/part1.rs

// bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes

use day_02::part1::sum_of_possible_game_ids;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = sum_of_possible_game_ids(input, 12, 13, 14);
    dbg!(output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_conundrum() {
        let input = "\
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(sum_of_possible_game_ids(input, 12, 13, 14), 8);
    }
}

// Define your structs and functions here...

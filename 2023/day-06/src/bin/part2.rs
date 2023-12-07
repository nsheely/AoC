// src/bin/part1.rs


use day_06::part2::number_of_ways_to_win;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = number_of_ways_to_win(input);
    dbg!(output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_races() {
        let input = "\
        Time:      7  15   30\n\
        Distance:  9  40  200";

        assert_eq!(number_of_ways_to_win(input), 71503);
    }
}

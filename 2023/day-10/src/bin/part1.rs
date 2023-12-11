// src/bin/part1.rs


use day_10::part1::find_longest_distance;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = find_longest_distance(input);
    dbg!(output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_loop() {
        let input = "\
            .....\n\
            .S-7.\n\
            .|.|.\n\
            .L-J.\n\
            .....";
        assert_eq!(find_longest_distance(input), 4);
    }

    #[test]
    fn complex_loop() {
        let input = "\
            ..F7.\n\
            .FJ|.\n\
            SJ.L7\n\
            |F--J\n\
            LJ...";
        assert_eq!(find_longest_distance(input), 8); // Replace with correct distance
    }
}


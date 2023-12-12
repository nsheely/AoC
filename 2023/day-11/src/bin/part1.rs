// src/bin/part1.rs


use day_11::part1::sum_of_pair_distances;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = sum_of_pair_distances(input);
    dbg!(output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_image() {
        let input = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....";
        assert_eq!(sum_of_pair_distances(input), 374);
    }
}


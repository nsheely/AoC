// src/bin/part2.rs

use day_13::part2::sum_of_pattern_notes;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = sum_of_pattern_notes(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_image() {
        let input = "\
        #.##..##.\n\
        ..#.##.#.\n\
        ##......#\n\
        ##......#\n\
        ..#.##.#.\n\
        ..##..##.\n\
        #.#.##.#.\n\
        \n\
        #...##..#\n\
        #....#..#\n\
        ..##..###\n\
        #####.##.\n\
        #####.##.\n\
        ..##..###\n\
        #....#..#";
        assert_eq!(sum_of_pattern_notes(input), 400);
    }
    
    #[test]
    fn in_test_1() {
        let input = "\
        ..#.....##.\n\
        .#..#..##.#\n\
        .......###.\n\
        ####.##.#.#\n\
        ###.###....\n\
        ###.###....\n\
        ####.##.#.#\n\
        .......####\n\
        .#..#..##.#\n\
        ..#.....##.\n\
        ..#.....##.\n\
        .#..#..##.#\n\
        .......####";
        assert_eq!(sum_of_pattern_notes(input), 500);
    }
    
    #[test]
    fn in_test_2() {
        let input = "\
        ##.#.##...####.\n\
        #####..#...##..\n\
        ##.##.#.#####.#\n\
        .##.#..##..##..\n\
        ###..#.#####.#.\n\
        ###..#.#####.#.\n\
        .##.#..##..##..\n\
        ##.##.#.#####.#\n\
        #####..#...##..\n\
        ##.#.##..#####.\n\
        ...#...#....###\n\
        ##.#.###.###..#\n\
        ..#.###.#.#....\n\
        ####...#.#.#...\n\
        .#..#....##.#.#\n\
        ....####.###.##\n\
        ....####.###.##";
        assert_eq!(sum_of_pattern_notes(input), 500);
    }

    #[test]
    fn in_test_3() {
        let input = "\
        ##.#...\n\
        .###...\n\
        ###...#\n\
        ##.###.\n\
        ...#...\n\
        ...#.#.\n\
        ###.###\n\
        ....#.#\n\
        ....#.#";
        assert_eq!(sum_of_pattern_notes(input), 1);
    }
}

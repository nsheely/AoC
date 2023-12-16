// src/bin/part1.rs

use day_13::part1::sum_of_pattern_notes;

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
        assert_eq!(sum_of_pattern_notes(input), 405);
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
        assert_eq!(sum_of_pattern_notes(input), 1000);// horizontal reflection between lines 10 and 11
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
        assert_eq!(sum_of_pattern_notes(input), 1600);// horizontal reflection betweens lines 4 and 5
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
        assert_eq!(sum_of_pattern_notes(input), 800); // horrizontal reflection between lines 8 and 9
    }
}

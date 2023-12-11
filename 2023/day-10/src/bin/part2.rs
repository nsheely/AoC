// src/bin/part2.rs


use day_10::part2::num_enclosed_tiles;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = num_enclosed_tiles(input);
    dbg!(output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "\
        ...........\n\
        .S-------7.\n\
        .|F-----7|.\n\
        .||.....||.\n\
        .||.....||.\n\
        .|L-7.F-J|.\n\
        .|..|.|..|.\n\
        .L--J.L--J.\n\
        ...........";
        assert_eq!(num_enclosed_tiles(input), 4);
    }

    #[test]
    fn example_2() {
        let input = "\
        .F----7F7F7F7F-7....\n\
        .|F--7||||||||FJ....\n\
        .||.FJ||||||||L7....\n\
        FJL7L7LJLJ||LJ.L-7..\n\
        L--J.L7...LJS7F-7L7.\n\
        ....F-J..F7FJ|L7L7L7\n\
        ....L7.F7||L7|.L7L7|\n\
        .....|FJLJ|FJ|F7|.LJ\n\
        ....FJL-7.||.||||...\n\
        ....L---J.LJ.LJLJ...";
        assert_eq!(num_enclosed_tiles(input), 8);
    }

    #[test]
    fn example_3() {
        let input = "\
        FF7FSF7F7F7F7F7F---7\n\
        L|LJ||||||||||||F--J\n\
        FL-7LJLJ||||||LJL-77\n\
        F--JF--7||LJLJIF7FJ-\n\
        L---JF-JLJIIIIFJLJJ7\n\
        |F|F-JF---7IIIL7L|7|\n\
        |FFJF7L7F-JF7IIL---7\n\
        7-L-JL7||F7|L7F-7F7|\n\
        L.L7LFJ|||||FJL7||LJ\n\
        L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(num_enclosed_tiles(input), 10);
    }
}


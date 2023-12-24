// src/bin/part1.rs

use day_17::part2::least_heat_loss;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = least_heat_loss(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_heat_loss_1() {
        let input = "\
            2413432311323\n\
            3215453535623\n\
            3255245654254\n\
            3446585845452\n\
            4546657867536\n\
            1438598798454\n\
            4457876987766\n\
            3637877979653\n\
            4654967986887\n\
            4564679986453\n\
            1224686865563\n\
            2546548887735\n\
            4322674655533";
        assert_eq!(least_heat_loss(input), 94);
    }

    #[test]
    fn test_least_heat_loss_2() {
        let input = "\
            111111111111\n\
            999999999991\n\
            999999999991\n\
            999999999991\n\
            999999999991";
        assert_eq!(least_heat_loss(input), 71);
    }
    
}

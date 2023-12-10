// src/bin/part1.rs


use day_08::part1::num_steps;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = num_steps(input);
    dbg!(output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "\
        RL\n\
        \n\
        AAA = (BBB, CCC)\n\
        BBB = (DDD, EEE)\n\
        CCC = (ZZZ, GGG)\n\
        DDD = (DDD, DDD)\n\
        EEE = (EEE, EEE)\n\
        GGG = (GGG, GGG)\n\
        ZZZ = (ZZZ, ZZZ)";
        assert_eq!(num_steps(input), 2);
    }

    #[test]
    fn example_2() {
        let input = "\
        LLR\n\
        \n\
        AAA = (BBB, BBB)\n\
        BBB = (AAA, ZZZ)\n\
        ZZZ = (ZZZ, ZZZ)";
        assert_eq!(num_steps(input), 6);
    }
}

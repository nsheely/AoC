// src/bin/part1.rs


use day_08::part2::num_steps;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = num_steps(input);
    dbg!(output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2() {
        let input = "\
            LR\n\
            \n\
            AAA = (ABB, XXX)\n\
            ABB = (XXX, AAZ)\n\
            AAZ = (ABB, XXX)\n\
            BBA = (BBB, XXX)\n\
            BBB = (BBC, BBC)\n\
            BBC = (BBZ, BBZ)\n\
            BBZ = (BBB, BBB)\n\
            XXX = (XXX, XXX)";

        assert_eq!(num_steps(input), 6);
    }

}

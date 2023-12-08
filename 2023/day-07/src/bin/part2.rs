// src/bin/part2.rs


use day_07::part2::total_winnings;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = total_winnings(input);
    dbg!(output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
        32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483";
        assert_eq!(total_winnings(input), 5905);
    }
}

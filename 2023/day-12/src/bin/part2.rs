// src/bin/part2.rs


use day_12::part2::sum_of_arrangements;

fn main() {
    let input = include_str!("../../input/input1.txt");
    let output = sum_of_arrangements(input);
    dbg!(output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_image() {
        let input = "\
        ???.### 1,1,3\n\
        .??..??...?##. 1,1,3\n\
        ?#?#?#?#?#?#?#? 1,3,1,6\n\
        ????.#...#... 4,1,1\n\
        ????.######..#####. 1,6,5\n\
        ?###???????? 3,2,1";
        assert_eq!(sum_of_arrangements(input), 525152);
    }
}


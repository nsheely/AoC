pub mod part1 {
    use rayon::prelude::*;
    // Counts the total number of valid arrangements for all rows in the input.
    pub fn sum_of_arrangements(input: &str) -> u64 {
        input
            .par_lines()
            .map(|line| {
                let (spring_layout, group_sizes) = line.split_once(' ').unwrap();
                let group_sizes = group_sizes
                    .split(',')
                    .map(|num| num.parse::<usize>().unwrap());
                count_arrangements(spring_layout, group_sizes)
            })
            .sum()
    }

    // Counts the number of valid arrangements for a single row.
    // `spring_layout` is the string representation of the springs with operational ('.'), damaged ('#'), or unknown ('?') status.
    // `group_sizes` is an iterator over the sizes of groups of damaged springs.
    fn count_arrangements(spring_layout: &str, group_sizes: impl Iterator<Item = usize>) -> u64 {
        // Collect group sizes into a vector for easy access
        let group_sizes = group_sizes.collect::<Vec<_>>();
        // Prepend a '.' to handle edge cases and trim trailing operational springs for optimization
        let spring_layout = format!(".{}", spring_layout.trim_end_matches('.'));
        // Convert the string into a character vector for easier iteration
        let spring_layout = spring_layout.chars().collect::<Vec<_>>();

        // Dynamic programming table to store the number of ways to arrange springs up to a certain point
        let mut dp = vec![0; spring_layout.len() + 1];
        // Base case: there's one way to arrange an empty set of springs
        dp[0] = 1;

        // Initialize the dp table for the first sequence of operational springs
        for (i, _) in spring_layout.iter().take_while(|&&c| c != '#').enumerate() {
            dp[i + 1] = 1;
        }

        // Process each group of damaged springs
        for count in group_sizes {
            // Temporary dp table for the current group of damaged springs
            let mut new_dp = vec![0; spring_layout.len() + 1];
            // Counter to track the length of the current sequence of damaged springs
            let mut current_sequence_length = 0;

            for (i, &c) in spring_layout.iter().enumerate() {
                if c != '.' {
                    // Increase the sequence length for a damaged spring
                    current_sequence_length += 1;
                } else {
                    // Reset the sequence length for an operational spring
                    current_sequence_length = 0;
                }

                // Carry over the number of ways from the previous spring if it's operational
                if c != '#' {
                    new_dp[i + 1] += new_dp[i];
                }

                // If the current sequence length matches the required count, 
                // add the number of ways from the dp table before this sequence started
                if current_sequence_length >= count && spring_layout[i - count] != '#' {
                    new_dp[i + 1] += dp[i - count];
                }
            }

            // Update the main dp table with the values calculated for the current group
            dp = new_dp;
        }

        // The last value in the dp table is the total number of valid arrangements for the row
        *dp.last().unwrap() as u64
    }
}

pub mod part2 {
    use itertools::Itertools;
    use rayon::prelude::*;

    // Counts the total number of valid arrangements for all unfolded rows in the input.
    pub fn sum_of_arrangements(input: &str) -> u128 {
        input
            .par_lines()
            .map(|line| {
                let (spring_layout, group_sizes) = line.split_once(' ').unwrap();
                
                // Calculate the required capacity to avoid reallocations
                let mut extended_spring_layout = String::with_capacity(spring_layout.len() * 5 + 4);
                
                // Build the string efficiently in one pass
                for i in 0..5 {
                    if i > 0 {
                        extended_spring_layout.push('?');
                    }
                    extended_spring_layout.push_str(spring_layout);
                }
    
                let group_sizes = group_sizes
                    .split(',')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect_vec();
                let n = group_sizes.len();
                count_arrangements(&extended_spring_layout, group_sizes.into_iter().cycle().take(5 * n))
            })
            .sum()
    }
    

    // Counts the number of valid arrangements for a single row.
    // `spring_layout` is the string representation of the springs with operational ('.'), damaged ('#'), or unknown ('?') status.
    // `group_sizes` is an iterator over the sizes of groups of damaged springs.
    fn count_arrangements(spring_layout: &str, group_sizes: impl Iterator<Item = usize>) -> u128 {
        // Collect group sizes into a vector for easy access
        let group_sizes = group_sizes.collect::<Vec<_>>();
        // Prepend a '.' to handle edge cases and trim trailing operational springs for optimization
        let spring_layout = format!(".{}", spring_layout.trim_end_matches('.'));
        // Convert the string into a character vector for easier iteration
        let spring_layout = spring_layout.chars().collect::<Vec<_>>();

        // Dynamic programming table to store the number of ways to arrange springs up to a certain point
        let mut dp = vec![0; spring_layout.len() + 1];
        // Base case: there's one way to arrange an empty set of springs
        dp[0] = 1;

        // Initialize the dp table for the first sequence of operational springs
        for (i, _) in spring_layout.iter().take_while(|&&c| c != '#').enumerate() {
            dp[i + 1] = 1;
        }

        // Process each group of damaged springs
        for count in group_sizes {
            // Temporary dp table for the current group of damaged springs
            let mut new_dp = vec![0; spring_layout.len() + 1];
            // Counter to track the length of the current sequence of damaged springs
            let mut current_sequence_length = 0;

            for (i, &c) in spring_layout.iter().enumerate() {
                if c != '.' {
                    // Increase the sequence length for a damaged spring
                    current_sequence_length += 1;
                } else {
                    // Reset the sequence length for an operational spring
                    current_sequence_length = 0;
                }

                // Carry over the number of ways from the previous spring if it's operational
                if c != '#' {
                    new_dp[i + 1] += new_dp[i];
                }

                // If the current sequence length matches the required count, 
                // add the number of ways from the dp table before this sequence started
                if current_sequence_length >= count && spring_layout[i - count] != '#' {
                    new_dp[i + 1] += dp[i - count];
                }
            }

            // Update the main dp table with the values calculated for the current group
            dp = new_dp;
        }

        // The last value in the dp table is the total number of valid arrangements for the row
        *dp.last().unwrap()
    }
}

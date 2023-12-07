pub mod part1 {
    // Function to calculate the product of the number of ways to win each race
    pub fn product_of_number_of_ways_to_win(input: &str) -> u32 {
        // Split input into lines for processing
        let mut lines = input.lines();

        // Extract times from the first line, skipping the label and parsing as floats
        let times = lines.next().unwrap_or("")
                          .split_whitespace()
                          .skip(1)
                          .filter_map(|s| s.parse::<f32>().ok());

        // Extract distances from the second line in a similar way
        let distances = lines.next().unwrap_or("")
                             .split_whitespace()
                             .skip(1)
                             .filter_map(|s| s.parse::<f32>().ok());

        // Zip times and distances, processing each race
        times.zip(distances)
             .map(|(time, distance)| {
                 // Apply quadratic formula to find valid hold times
                 let delta_sqrt = (time * time - 4.0 * distance).sqrt();
                 let x1 = ((time + delta_sqrt) / 2.0 - 1.0).ceil();  // Upper bound
                 let x2 = ((time - delta_sqrt) / 2.0 + 1.0).floor(); // Lower bound

                 // Count of winning hold times for each race
                 (x1 - x2 + 1.0) as u32
             })
             .product() // Product of counts across all races
    }
}

pub mod part2 {
    // Function to calculate the number of ways to win a single long race
    pub fn number_of_ways_to_win(input: &str) -> u64 {
        // Split input into lines for processing
        let mut lines = input.lines();

        // Parse concatenated time and distance numbers
        let time = parse_concatenated_numbers(lines.next().unwrap_or(""));
        let distance = parse_concatenated_numbers(lines.next().unwrap_or(""));

        // Apply the quadratic formula to calculate valid hold times
        let time = time as f64;
        let distance = distance as f64;
        let delta_sqrt = (time.powi(2) - 4.0 * distance).sqrt();
        let x1 = ((time + delta_sqrt) / 2.0 - 1.0).ceil();  // Upper bound
        let x2 = ((time - delta_sqrt) / 2.0 + 1.0).floor(); // Lower bound

        // Return the number of valid hold times for the race
        ((x1 - x2 + 1.0) as u64).max(0)
    }

    // Function to parse concatenated numbers from a string line
    fn parse_concatenated_numbers(line: &str) -> u64 {
        line.split_whitespace()
            .skip(1) // Skip the label
            .flat_map(|num_str| num_str.chars())
            .fold(0, |acc, digit| {
                // Convert each character to a digit and concatenate
                acc * 10 + digit.to_digit(10).unwrap() as u64
            })
    }
}

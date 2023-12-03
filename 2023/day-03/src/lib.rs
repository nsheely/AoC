pub mod part1 {
    use std::collections::VecDeque;

    // Extracts numbers and symbols from a line of the schematic.
    // Numbers are stored as tuples with start index, end index, and value.
    // Symbols are stored as their indices.
    fn extract_numbers_and_symbols(line: &str) -> (Vec<(usize, usize, u32)>, Vec<usize>) {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();
        let mut start_index = None;

        for (i, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                // Start recording a number if not already started.
                start_index.get_or_insert(i);
            } else {
                if let Some(start) = start_index {
                    // If a number was being recorded, end and save it.
                    let number: u32 = line[start..=i - 1].parse().unwrap();
                    numbers.push((start, i - 1, number));
                    start_index = None;
                }
                // Record symbols other than '.' by their indices.
                if ch != '.' {
                    symbols.push(i);
                }
            }
        }

        // Handle the last number if the line ends with a number.
        if let Some(start) = start_index {
            let number: u32 = line[start..].parse().unwrap();
            numbers.push((start, line.len() - 1, number));
        }

        (numbers, symbols)
    }

    // Sums the values of numbers that are adjacent to symbols.
    fn sum_numbers_with_adjacent_symbols(
        numbers: &[(usize, usize, u32)], // List of numbers in the current line with their start and end indices.
        prev_symbols: &[usize],          // List of symbol indices in the previous line.
        current_symbols: &[usize],       // List of symbol indices in the current line.
        next_symbols: &[usize],          // List of symbol indices in the next line.
    ) -> u32 {
        let mut sum = 0;

        // Indices to keep track of our position in the symbol arrays as we iterate.
        let mut prev_index = 0;
        let mut curr_index = 0;
        let mut next_index = 0;

        // Iterate through each number in the current line.
        for &(start, end, number) in numbers {
            // Define the range to check for adjacent symbols, including diagonals.
            let range_start = start.saturating_sub(1);
            let range_end = end + 1;

            // Check if there are adjacent symbols in any of the three lines.
            let found_adjacent =
                check_symbols_in_range(prev_symbols, range_start, range_end, &mut prev_index)
                    || check_symbols_in_range(
                        current_symbols,
                        range_start,
                        range_end,
                        &mut curr_index,
                    )
                    || check_symbols_in_range(
                        next_symbols,
                        range_start,
                        range_end,
                        &mut next_index,
                    );

            // Add to the sum if an adjacent symbol was found.
            if found_adjacent {
                sum += number;
            }
        }

        sum
    }

    // Checks if there are any symbols within a specific range in a line.
    fn check_symbols_in_range(
        symbols: &[usize],
        range_start: usize,
        range_end: usize,
        index: &mut usize,
    ) -> bool {
        // Advance the index to skip symbols that are before the range.
        while *index < symbols.len() && symbols[*index] < range_start {
            *index += 1;
        }

        // Check if the current symbol falls within the range.
        if *index < symbols.len() && symbols[*index] <= range_end {
            return true; // Found an adjacent symbol
        }

        false
    }

    // Main function to sum part numbers in the schematic.
    // Uses a VecDeque to keep track of the current line and its adjacent lines.
    pub fn sum_of_part_numbers(input: &str) -> u32 {
        let mut lines = VecDeque::new();
        lines.push_back(extract_numbers_and_symbols("")); // Dummy line for the start

        let mut total_sum = 0;

        // Iterate over each line and the dummy line at the end.
        for line in input.lines().chain(std::iter::once("")) {
            let (current_numbers, current_symbols) = extract_numbers_and_symbols(line);
            lines.push_back((current_numbers, current_symbols));

            // Once we have three lines (previous, current, next), process the current lines numbers against symbols in each line.
            if lines.len() == 3 {
                let (_, prev_symbols) = &lines[0];
                let (current_numbers, current_symbols) = &lines[1];
                let (_, next_symbols) = &lines[2];

                // Sum numbers adjacent to symbols and add to the total sum.
                total_sum += sum_numbers_with_adjacent_symbols(
                    current_numbers,
                    prev_symbols,
                    current_symbols,
                    next_symbols,
                );

                // Remove the oldest line to make room for the next line.
                lines.pop_front();
            }
        }

        total_sum
    }
}

pub mod part2 {
    use std::collections::VecDeque;

    fn extract_numbers_and_gears(line: &str) -> (Vec<(usize, usize, u32)>, Vec<usize>) {
        let mut numbers = Vec::new();
        let mut gears = Vec::new();
        let mut start_index = None;

        for (i, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                // Start recording a number if not already started.
                start_index.get_or_insert(i);
            } else {
                if let Some(start) = start_index {
                    // If a number was being recorded, end and save it.
                    let number: u32 = line[start..=i - 1].parse().unwrap();
                    numbers.push((start, i - 1, number));
                    start_index = None;
                }
                // Record gears by their indices.
                if ch == '*' {
                    gears.push(i);
                }
            }
        }

        // Handle the last number if the line ends with a number.
        if let Some(start) = start_index {
            let number: u32 = line[start..].parse().unwrap();
            numbers.push((start, line.len() - 1, number));
        }

        (numbers, gears)
    }

    // Function to sum gear ratios
    fn sum_gear_ratios(
        gears: &[usize],                         // Gears in the current line
        prev_numbers: &[(usize, usize, u32)],    // Numbers in the previous line
        current_numbers: &[(usize, usize, u32)], // Numbers in the current line
        next_numbers: &[(usize, usize, u32)],    // Numbers in the next line
    ) -> u32 {
        let mut sum = 0;

        // Indices to keep track of our position in the symbol arrays as we iterate.
        let mut prev_index = 0;
        let mut curr_index = 0;
        let mut next_index = 0;

        for &gear_index in gears {
            // Gather adjacent numbers to this gear
            let mut adjacent_numbers = Vec::new();
            adjacent_numbers.extend(find_adjacent_numbers(
                gear_index,
                prev_numbers,
                &mut prev_index,
            ));
            adjacent_numbers.extend(find_adjacent_numbers(
                gear_index,
                next_numbers,
                &mut next_index,
            ));
            if adjacent_numbers.len() <= 2 {
                adjacent_numbers.extend(find_adjacent_numbers(
                    gear_index,
                    current_numbers,
                    &mut curr_index,
                ));
            }

            // Calculate gear ratio if exactly two numbers are found
            if adjacent_numbers.len() == 2 {
                sum += adjacent_numbers[0].2 * adjacent_numbers[1].2;
            }
        }

        sum
    }

    // Helper function to find adjacent numbers to a given index
    fn find_adjacent_numbers(
        gear_index: usize,
        numbers: &[(usize, usize, u32)],
        index: &mut usize,
    ) -> Vec<(usize, usize, u32)> {
        let mut adjacent_numbers = Vec::new();
    
        // Advance the index to the first number that could be adjacent to the gear.
        while *index < numbers.len() && numbers[*index].1 < gear_index.saturating_sub(1) {
            *index += 1;
        }
    
        // Check numbers in the range of this gear.
        while *index < numbers.len() {
            let (start, end, value) = numbers[*index];
            if end < gear_index.saturating_sub(1) {
                // If the end index is before the gear, continue to the next number.
                *index += 1;
                continue;
            }
    
            if start > gear_index + 1 {
                // If the start index is past the gear, stop checking further numbers.
                break;
            }
    
            if (start..=end).any(|i| (i as i32 - gear_index as i32).abs() <= 1) {
                // If the number is adjacent to the gear, add it to the list.
                adjacent_numbers.push((start, end, value));
                if adjacent_numbers.len() == 2 {
                    // Only need two adjacent numbers for a gear.
                    break;
                }
            }
    
            *index += 1;
        }
    
        adjacent_numbers
    }
    
    

    // Main function to calculate the sum of all gear ratios
    pub fn sum_of_gear_ratios(input: &str) -> u32 {
        let mut lines = VecDeque::new();
        lines.push_back(extract_numbers_and_gears(""));

        let mut total_sum = 0;

        for line in input.lines().chain(std::iter::once("")) {
            let (current_numbers, current_gears) = extract_numbers_and_gears(line);
            lines.push_back((current_numbers, current_gears));

            if lines.len() == 3 {
                let (prev_numbers, _) = &lines[0];
                let (current_numbers, current_gears) = &lines[1];
                let (next_numbers, _) = &lines[2];

                total_sum +=
                    sum_gear_ratios(current_gears, prev_numbers, current_numbers, next_numbers);
                lines.pop_front();
            }
        }

        total_sum
    }
}

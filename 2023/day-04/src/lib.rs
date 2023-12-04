pub mod part1 {
    const MAX_NUMBER: usize = 100; // Maximum expected number

    pub fn scratchcard_winnings_sum(input: &str) -> u32 {
        input.lines()
            .map(|line| {
                let mut score = 0;
                let mut winning_numbers = [false; MAX_NUMBER];
                let mut player_numbers_started = false;

                // Skip the "Card X: " part
                let numbers_part = line.split_once(": ").unwrap().1;

                for word in numbers_part.split_whitespace() {
                    // Detect when player numbers start
                    if word == "|" {
                        player_numbers_started = true;
                        continue;
                    }

                    // Parse the number using custom lightweight parser
                    let number = parse_small_number(word);

                    if player_numbers_started {
                        if winning_numbers[number] {
                            score = if score == 0 { 1 } else { score * 2 };
                        }
                    } else {
                        winning_numbers[number] = true;
                    }
                }

                score
            })
            .sum()
    }

    // Custom parser for small numbers (less than 100)
    fn parse_small_number(s: &str) -> usize {
        let bytes = s.as_bytes();
        let mut result = (bytes[0] - b'0') as usize;
        if bytes.len() == 2 {
            result = result * 10 + (bytes[1] - b'0') as usize;
        }
        result
    }
}

pub mod part2 {
    const MAX_NUMBER: usize = 100; // Maximum expected number

    pub fn total_scratchcards(input: &str) -> u32 {
        let lines: Vec<&str> = input.lines().collect();
        let mut card_counts = vec![1u32; lines.len()]; // Initialize with 1 for each card

        for (card_index, line) in lines.iter().enumerate() {
            let mut winning_numbers = [false; MAX_NUMBER];
            let mut player_numbers_started = false;
            let mut match_count = 0;
            // Skip the "Card X: " part
            let numbers_part = line.split_once(": ").unwrap().1;

            for word in numbers_part.split_whitespace() {
                // Detect when player numbers start
                if word == "|" {
                    player_numbers_started = true;
                    continue;
                }

                // Parse the number using custom lightweight parser
                let number = parse_small_number(word);

                if player_numbers_started {
                    if winning_numbers[number] {
                        // Add copies for each subsequent card equal to the number of matches
                        match_count += 1;
                        card_counts[card_index + match_count] += card_counts[card_index];
                    }
                } else {
                    winning_numbers[number] = true;
                }
            }
        }

        card_counts.iter().sum() // Sum up the total number of cards
    }

    // Custom parser for small numbers (less than 100)
    fn parse_small_number(s: &str) -> usize {
        let bytes = s.as_bytes();
        let mut result = (bytes[0] - b'0') as usize;
        if bytes.len() == 2 {
            result = result * 10 + (bytes[1] - b'0') as usize;
        }
        result
    }
}

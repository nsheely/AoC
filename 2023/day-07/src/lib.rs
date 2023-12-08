pub mod part1 {
    struct Hand {
        bid: u32,
        strength: u32,
    }

    impl Hand {
        // Create a new Hand from the given string of cards and bid amount.
        #[inline]
        fn new(cards_str: &str, bid: u32) -> Self {
            let mut card_strength: u32 = 0;
            let mut cards: [u32; 13] = [0; 13];

            // Iterate over each card character in the hand string.
            for (i, card) in (0_u32..).zip(cards_str.chars()) {
                // Determine the value of the card.
                let val = match card {
                    'A' => 12,
                    'K' => 11,
                    'Q' => 10,
                    'J' => 9,
                    'T' => 8,
                    n => n.to_digit(10).unwrap() - 2,
                };

                // Increment the count for this card value.
                cards[val as usize] += 1;

                // Calculate the part of the strength based on this card's value.
                card_strength += val * (1 << ((4 - i) * 4));
            }

            // Determine the hand type based on the counts of each card.
            let mut hand_type = 0;
            let mut pair_count = 0;
            let mut three_of_a_kind = false;

            for &count in &cards {
                match count {
                    4 => hand_type = 5, // Four of a kind.
                    3 => {
                        // Three of a kind; check if there is also a pair for a full house.
                        if pair_count > 0 {
                            hand_type = 4; // Full house.
                        } else {
                            three_of_a_kind = true;
                        }
                    },
                    2 => {
                        // One pair; check if there is another pair or a three of a kind.
                        pair_count += 1;
                        if pair_count == 2 {
                            hand_type = 2; // Two pair.
                        } else if three_of_a_kind {
                            hand_type = 4; // Full house.
                        } else {
                            hand_type = 1; // One pair.
                        }
                    },
                    _ => {} // No special hand type for this count.
                }
            }
    
            // If no specific hand type is found, check for three of a kind.
            if hand_type == 0 && three_of_a_kind {
                hand_type = 3; // Three of a kind.
            }
    
            // Combine hand type and card strength to get the final strength value.
            let strength = (hand_type << 20) + card_strength;
    
            Hand { bid, strength }
        }
    }

    // Public function to calculate total winnings given a string input representing hands and bids.
    pub fn total_winnings(input: &str) -> usize {
        calculate_winnings(input)
    }

    // Function to calculate winnings, taking a string input.
    fn calculate_winnings(input: &str) -> usize {
        let mut hands: Vec<Hand> = Vec::with_capacity(input.lines().count());

        // Process each line to create a Hand and add it to the vector.
        for line in input.lines() {
            let mut split = line.split_whitespace();
            let cards_str = split.next().unwrap();
            let bid: u32 = split.next().unwrap().parse().unwrap();

            hands.push(Hand::new(cards_str, bid));
        }

        // Sort the hands based on their strength.
        hands.sort_unstable_by_key(|hand| hand.strength);

        // Calculate the total winnings based on the sorted hands and their bids.
        hands
            .iter()
            .enumerate()
            .fold(0, |acc, (i, hand)| acc + ((i + 1) * hand.bid as usize))
    }
}


pub mod part2 {
    // Define a struct to represent a hand of cards in the game.
    struct Hand {
        bid: u32,       // The bid amount associated with the hand.
        strength: u32,  // The calculated strength of the hand based on card values and jokers.
    }
    
    impl Hand {
        // Create a new Hand from the given string of cards and bid amount.
        #[inline]
        fn new(cards_str: &str, bid: u32) -> Self {
            let mut strength: u32 = 0; // Initialize the strength of the hand.
            let mut jokers = 0;        // Counter for the number of jokers in the hand.
            let mut cards: [u32; 13] = [0; 13]; // Array to track the count of each card value.

            // Iterate over each card character and calculate the card values and strength.
            cards_str.chars().enumerate().for_each(|(i, card)| {
                let val = match card {
                    // Assign values to each card, with Jokers being counted separately.
                    'A' => 12,
                    'K' => 11,
                    'Q' => 10,
                    'J' => { jokers += 1; 0 },
                    'T' => 9,
                    n => n.to_digit(10).unwrap() - 1,
                };
    
                // Increment the count for each card value.
                if val != 0 {
                    cards[val as usize] += 1;
                }
    
                // Calculate the part of the strength based on this card's value.
                strength |= val << ((4 - i) * 4);
            });

            // Sort the card counts to help determine the hand type.
            cards.sort_unstable();

            // Determine the hand type, considering jokers for their ability to represent any card.
            let hand_type = match cards[12] + jokers {
                5 => 6,  // Five of a kind.
                4 => 5,  // Four of a kind.
                3 if cards[11] == 2 => 4,  // Full house.
                3 => 3,  // Three of a kind.
                2 if cards[11] == 2 => 2,  // Two pair.
                2 => 1,  // One pair.
                _ => 0,  // High card.
            };

            // Combine the hand type and the card strength to calculate the final strength of the hand.
            strength |= hand_type << 20;

            // Return the Hand struct with the calculated strength and bid.
            Hand { bid, strength }
        }
    }
    
    // Function to calculate total winnings given a string input representing hands and bids.
    pub fn total_winnings(input: &str) -> usize {
        calculate_winnings(input)
    }

    // Function to calculate winnings based on the hand strength and bid values.
    fn calculate_winnings(input: &str) -> usize {
        // Pre-allocate a vector for the hands based on the number of lines in the input.
        let mut hands: Vec<Hand> = Vec::with_capacity(input.lines().count());

        // Process each line to create a Hand and add it to the vector.
        for line in input.lines() {
            let mut split = line.split_whitespace();
            let cards_str = split.next().unwrap();
            let bid: u32 = split.next().unwrap().parse().unwrap();

            // Create a new Hand from the card string and bid, and add it to the collection.
            hands.push(Hand::new(cards_str, bid));
        }

        // Sort the hands based on their calculated strength.
        hands.sort_unstable_by_key(|hand| hand.strength);

        // Calculate the total winnings by iterating over the sorted hands.
        // The rank of each hand is determined by its position in the sorted array.
        hands
            .iter()
            .enumerate()
            .fold(0, |acc, (i, hand)| acc + ((i + 1) * hand.bid as usize))
    }
}


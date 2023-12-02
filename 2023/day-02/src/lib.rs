
pub mod part1 {
    // Checks if a single pull of cubes from the bag is valid given the maximum allowed cubes of each color.
    fn is_valid_pull(segment: &str, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        for color_info in segment.split(',') {
            let mut color_parts = color_info.split_whitespace();
            let count = color_parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
            match color_parts.next().unwrap_or("") {
                "red" if count > max_red => return false,
                "green" if count > max_green => return false,
                "blue" if count > max_blue => return false,
                _ => (),
            }
        }
        true
    }

    // Parses a game line and returns the game id if each pull is valid, otherwise None.
    fn parse_and_validate_game(line: &str, max_red: u32, max_green: u32, max_blue: u32) -> Option<u32> {
        let (id_part, segments) = line.split_once(':')?;
        let game_id = id_part.split_whitespace().last()?.parse::<u32>().ok()?;
        
        for segment in segments.split(';') {
            if !is_valid_pull(segment, max_red, max_green, max_blue) {
                return None;
            }
        }
        Some(game_id)
    }

    // Calculates the sum of game IDs that are possible with given max cube counts.
    pub fn sum_of_possible_game_ids(input: &str, max_red: u32, max_green: u32, max_blue: u32) -> u32 {
        input
            .lines()
            .filter_map(|line| parse_and_validate_game(line, max_red, max_green, max_blue))
            .sum()
    }
}

pub mod part2 {
    // Extracts the number of each color of cube pulled in a single segment.
    fn pull_tuple(segment: &str) -> (u32, u32, u32) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for color_info in segment.split(',') {
            let mut color_parts = color_info.split_whitespace();
            let count = color_parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
            match color_parts.next().unwrap_or("") {
                "red" => red = red.max(count),
                "green" => green = green.max(count),
                "blue" => blue = blue.max(count),
                _ => (),
            }
        }

        (red, green, blue)
    }

    // Parses a game line and calculates the product of the maximum counts of each color cube pulled.
    fn parse_and_min_product_game(line: &str) -> Option<u32> {
        let (id_part, segments) = line.split_once(':')?;
        let _game_id = id_part.split_whitespace().last()?.parse::<u32>().ok()?;

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for segment in segments.split(';') {
            let (red, green, blue) = pull_tuple(segment);
            max_red = max_red.max(red);
            max_green = max_green.max(green);
            max_blue = max_blue.max(blue);
        }

        Some(max_red * max_green * max_blue)
    }

    // Calculates the sum of the product of the maximum counts of each color cube for all games.
    pub fn sum_of_product_of_min_cubes(input: &str) -> u32 {
        input
            .lines()
            .filter_map(parse_and_min_product_game)
            .sum()
    }
}

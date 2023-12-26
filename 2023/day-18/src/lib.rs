pub mod part1 {
    /// Function to calculate the cubic meters of lava the lagoon can hold based on the dig plan.
    pub fn calculate_lava_capacity(dig_plan: &str) -> i64 {
        let mut x: i64 = 0;
        let mut area: i64 = 0;
        let mut perimeter: i64 = 0;

        for line in dig_plan.lines() {
            let mut parts = line.split_whitespace();
            let direction = parts.next().and_then(|d| d.chars().next()).unwrap_or_default();
            let length = parts.next().and_then(|l| l.parse::<i64>().ok()).unwrap_or_default();

            match direction {
                'R' => {
                    perimeter += length;
                    x += length;
                },
                'L' => {
                    perimeter += length;
                    x -= length;
                },
                'D' => {
                    perimeter += length;
                    area += x * length; // Adding rectangles formed by vertical movement
                },
                'U' => {
                    perimeter += length;
                    area -= x * length; // Subtracting rectangles formed by vertical movement
                },
                _ => {}
            }
        }

        // Adjusting the area calculation to include the interior
        area + perimeter / 2 + 1
    }
}

pub mod part2 {
    /// Function to calculate the cubic meters of lava the lagoon can hold based on the corrected dig plan.
    pub fn calculate_lava_capacity(dig_plan: &str) -> i64 {
        let mut x: i64 = 0;
        let mut area: i64 = 0;
        let mut perimeter: i64 = 0;

        for line in dig_plan.lines() {
            if let Some(start) = line.find('#') {
                let hex_code = &line[start + 1..];

                let length = match i64::from_str_radix(&hex_code[..5], 16) {
                    Ok(l) => l,
                    Err(_) => continue, // Skip if the length part is not valid
                };

                let direction = match hex_code.chars().nth(5) {
                    Some('0') => 'R',
                    Some('1') => 'D',
                    Some('2') => 'L',
                    Some('3') => 'U',
                    _ => continue, // Skip if the direction is not valid
                };

                match direction {
                    'R' => {
                        perimeter += length;
                        x += length;
                    },
                    'L' => {
                        perimeter += length;
                        x -= length;
                    },
                    'D' => {
                        perimeter += length;
                        area += x * length;
                    },
                    'U' => {
                        perimeter += length;
                        area -= x * length;
                    },
                    _ => {}
                }
            }
        }

        area + perimeter / 2 + 1
    }
}



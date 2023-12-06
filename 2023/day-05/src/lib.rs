pub mod part1 {
    use std::collections::BTreeMap;

    // Define the structure `Almanac` to hold the mappings.
    pub struct Almanac {
        // An array of BTreeMaps, each representing a different category of mapping.
        // BTreeMap is chosen for efficient range queries.
        mappings: [BTreeMap<u64, (u64, u64)>; 7],
    }

    impl Almanac {
        // Constructs a new `Almanac` from a string input.
        fn new(input: &str) -> Almanac {
            let lines = input.lines(); // Split the input into lines.
            // Initialize an array of BTreeMaps, one for each category.
            let mut mappings = [BTreeMap::new(), BTreeMap::new(), BTreeMap::new(), BTreeMap::new(), BTreeMap::new(), BTreeMap::new(), BTreeMap::new()];
            let mut current_map = 0; // Index to track the current map being processed.

            // Iterate through each line in the input.
            for line in lines {
                if line.contains("map:") {
                    // Increment the index when a new category is encountered.
                    current_map += 1;
                } else if !line.is_empty() {
                    // Parse the mapping line into a vector of parts.
                    let parts: Vec<u64> = line.split_whitespace()
                                              .filter_map(|s| s.parse().ok())
                                              .collect();
                    // Insert the mapping into the appropriate BTreeMap.
                    if parts.len() == 3 {
                        mappings[current_map - 1].insert(parts[1], (parts[0], parts[2]));
                    }
                }
            }

            Almanac { mappings }
        }

        // Processes a single seed through all the mappings to determine its final value.
        fn process_seed(&self, seed: u64) -> u64 {
            // Iterate over each mapping category in order, applying the mapping to the seed.
            self.mappings.iter().fold(seed, |acc, map| {
                // For each category, find the appropriate range and compute the mapped value.
                map.range(..=acc).next_back()
                   .map_or(acc, |(&src_start, &(dest_start, length))| {
                       if acc < src_start + length {
                           // If the seed is within the range, calculate the mapped value.
                           dest_start + (acc - src_start)
                       } else {
                           // If not within any range, return the seed as is.
                           acc
                       }
                   })
            })
        }
    }

    // Computes the lowest location number from the given input.
    pub fn find_lowest_location(input: &str) -> u64 {
        let mut lines = input.lines(); // Split the input into lines.
        // Parse the first line to extract the seed values.
        let seeds_line = lines.next().unwrap_or("");
        let seeds: Vec<u64> = seeds_line.split(": ")
            .nth(1).unwrap_or("")
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Create an Almanac from the input.
        let almanac = Almanac::new(input);

        // Iterate over the seeds, process each through the Almanac, and find the minimum.
        seeds.into_iter()
             .map(|seed| almanac.process_seed(seed))
             .min()
             .unwrap_or(u64::MAX)
    }
}

pub mod part2 {
    use std::collections::BTreeMap;

    // Define the structure `Almanac` to hold the mappings.
    pub struct Almanac {
        // An array of BTreeMaps, each representing a different category of mapping.
        // BTreeMap is chosen for efficient range queries.
        mappings: [BTreeMap<u64, (u64, u64)>; 7],
    }

    impl Almanac {
        // Constructs a new `Almanac` from a string input.
        fn new(input: &str) -> Almanac {
            let lines = input.lines(); // Split the input into lines.
            // Initialize an array of BTreeMaps, one for each category.
            let mut mappings = [BTreeMap::new(), BTreeMap::new(), BTreeMap::new(), BTreeMap::new(), BTreeMap::new(), BTreeMap::new(), BTreeMap::new()];
            let mut current_map = 0; // Index to track the current map being processed.

            // Iterate through each line in the input.
            for line in lines {
                if line.contains("map:") {
                    // Increment the index when a new category is encountered.
                    current_map += 1;
                } else if !line.is_empty() {
                    // Parse the mapping line into a vector of parts.
                    let parts: Vec<u64> = line.split_whitespace()
                                              .filter_map(|s| s.parse().ok())
                                              .collect();
                    // Insert the mapping into the appropriate BTreeMap.
                    if parts.len() == 3 {
                        mappings[current_map - 1].insert(parts[1], (parts[0], parts[2]));
                    }
                }
            }

            Almanac { mappings }
        }

        // Processes a given range of seed numbers through all mappings.
        fn process_seed_range(&self, start: u64, length: u64) -> u64 {
            // Start with the initial seed range.
            let mut ranges = vec![(start, start + length)];
            // Apply each mapping to the range.
            for map in self.mappings.iter() {
                // Each range is remapped according to the current mapping.
                ranges = ranges.into_iter()
                    .flat_map(|range| Self::remap_range(range, map))
                    .collect();
            }
            // Find the minimum start value from all processed ranges.
            ranges.into_iter().map(|(start, _)| start).min().unwrap_or(u64::MAX)
        }

        // Applies a given mapping to a range and returns the resulting ranges.
        fn remap_range(range: (u64, u64), map: &BTreeMap<u64, (u64, u64)>) -> Vec<(u64, u64)> {
            let (start, end) = range;
            let mut result = Vec::new();
            let mut current_start = start;
    
            // Iterate over the range, applying the mapping to each part.
            while current_start < end {
                // Find the mapping that applies to the current start of the range.
                if let Some((&src_start, &(dest_start, range_len))) = map.range(..=current_start).next_back() {
                    let src_end = src_start + range_len;
                    // If the current start is within a mapped range, calculate the new range.
                    if current_start < src_end {
                        let new_start = dest_start + (current_start - src_start);
                        let new_end = dest_start + (src_end.min(end) - src_start);
                        result.push((new_start, new_end));
                        current_start = src_end;
                    } else {
                        // If no overlap, push the original range and break.
                        result.push((current_start, end));
                        break;
                    }
                } else {
                    // If no mapping for this range, push the original range and break.
                    result.push((current_start, end));
                    break;
                }
            }
    
            result
        }    
        
    }

    // Finds the lowest location number from all the initial seed ranges.
    pub fn find_lowest_location(input: &str) -> u64 {
        let mut lines = input.lines();
        // Parse the initial seed ranges from the first line of the input.
        let seeds_line = lines.next().unwrap_or("");
        let seed_ranges: Vec<(u64, u64)> = seeds_line.split(": ")
            .nth(1).unwrap_or("")
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<u64>>()
            .chunks(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();

        let almanac = Almanac::new(input);

        // Process each seed range through the almanac and find the minimum result.
        seed_ranges.into_iter()
            .map(|(start, length)| almanac.process_seed_range(start, length))
            .min()
            .unwrap_or(u64::MAX)
    }
}


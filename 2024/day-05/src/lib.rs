pub mod part1 {
    pub fn sum_middle_pages_of_correct_updates(input: &str) -> u32 {
        const MAX_PAGE: usize = 100;
        // Use u8 instead of bool to save memory bandwidth
        let mut before = [[0u8; MAX_PAGE]; MAX_PAGE];
        let mut sum = 0;
    
        // Split and get sections
        let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    
        // Process rules
        for line in rules_str.lines() {
            let (x, y) = line.split_once('|').unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            before[x][y] = 1;
        }
    
        // Build transitive closure with early exit
        for k in 0..MAX_PAGE {
            let mut changed = false;
            for i in 0..MAX_PAGE {
                if before[i][k] == 1 {
                    for j in 0..MAX_PAGE {
                        if before[k][j] == 1 && before[i][j] == 0 {
                            before[i][j] = 1;
                            changed = true;
                        }
                    }
                }
            }
            if !changed { break; }
        }
    
        // Process updates
        for line in updates_str.lines() {
            let update: Vec<usize> = line
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            
            let mut valid = true;
            'outer: for i in 0..update.len() {
                for j in i+1..update.len() {
                    if before[update[j]][update[i]] == 1 {
                        valid = false;
                        break 'outer;
                    }
                }
            }
    
            if valid {
                sum += update[update.len()/2] as u32;
            }
        }
    
        sum
    }
}

pub mod part2 {
    pub fn sum_middle_pages_of_corrected_updates(input: &str) -> u32 {
        const MAX_PAGE: usize = 100;
        let mut before = [[0u8; MAX_PAGE]; MAX_PAGE];
        let mut sum = 0;
    
        // Split and get sections
        let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    
        // Process rules
        for line in rules_str.lines() {
            let (x, y) = line.split_once('|').unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            before[x][y] = 1;
        }
    
        // Build transitive closure
        for k in 0..MAX_PAGE {
            let mut changed = false;
            for i in 0..MAX_PAGE {
                if before[i][k] == 1 {
                    for j in 0..MAX_PAGE {
                        if before[k][j] == 1 && before[i][j] == 0 {
                            before[i][j] = 1;
                            changed = true;
                        }
                    }
                }
            }
            if !changed { break; }
        }
    
        // Process updates
        for line in updates_str.lines() {
            let mut update: Vec<usize> = line
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            
            // Check if update is incorrectly ordered
            let mut valid = true;
            for i in 0..update.len() {
                for j in i+1..update.len() {
                    if before[update[j]][update[i]] == 1 {
                        valid = false;
                        break;
                    }
                }
            }
    
            if !valid {
                // Sort using the ordering rules
                update.sort_by(|&a, &b| {
                    if before[b][a] == 1 {
                        std::cmp::Ordering::Greater
                    } else if before[a][b] == 1 {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
                
                sum += update[update.len()/2] as u32;
            }
        }
    
        sum
    }
}
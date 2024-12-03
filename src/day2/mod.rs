use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn first(path: String) {
    println!("Reading file {}", path);
    let file = match File::open(Path::new(path.as_str())) {
        Err(why) => panic!("Couldn't open {}: {}", path, why),
        Ok(file) => file,
    };
    let reader = io::BufReader::new(file);
    let mut safe_count = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line
            .split_whitespace()
            .map(|part| part.parse::<i32>().unwrap());
        let mut current = parts.next().unwrap();
        let next = parts.next().unwrap();
        let difference = next - current;
        let direction = difference.signum();
        let mut safe = 1;

        if difference.abs() < 1 || difference.abs() > 3 {
            continue;
        }

        current = next;
        for next in parts {
            let difference = next - current;
            if difference.abs() < 1 || difference.abs() > 3 || difference.signum() != direction {
                safe = 0;
                break;
            }
            current = next;
        }
        safe_count += safe;
    }
    println!("Safe count: {}", safe_count);
}

/*
* The tricky bit about part two is being able to skip one element and restoring the state to what
* it was before that element.
*
* For instance, the sequence 5 6 3 2 1 is safe, but the second element sets an ascending direction,
* which isn't consistent with the third and subsequent elements. We need to be able to skip
* elements that seem initially safe.
*
* Because we only allow at most one skippable element, it should be enough to backtrace one step.
*/
fn is_safe<'a, I>(mut values: I) -> bool
where
    I: Iterator<Item = &'a i32>,
{
    let mut direction = 0;
    let mut skipped = false;
    let mut previous_direction = 0;
    let mut previous = &0;
    let mut current = values.next().unwrap();
    for next in values {
        let difference = (next - current).abs();
        let next_direction = (next - current).signum();

        if difference.abs() < 1 || difference > 3 {
            if skipped {
                return false;
            }
            skipped = true;
            continue;
        }

        if direction != 0 && next_direction != direction {
            if skipped {
                return false;
            }
            // We might be the bad element or the previous one was bad and set the wrong direction
            if previous_direction == 0 {
                // The previous element set the direction, assume it was wrong and skip that one
                direction = next_direction;
                current = previous;
            }
            skipped = true;
            continue;
        }

        previous = current;
        previous_direction = direction;
        if direction == 0 {
            direction = next_direction;
        }
        current = next
    }
    true
}

pub fn second(path: String) {
    println!("Reading file {}", path);
    let file = match File::open(Path::new(path.as_str())) {
        Err(why) => panic!("Couldn't open {}: {}", path, why),
        Ok(file) => file,
    };
    let reader = io::BufReader::new(file);

    let mut safe_count = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        // I wrote is_safe to take an iterator, but I can't get the reference types to work, so I'm
        // calling collect and building a second iterator instead :_(
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|part| part.parse::<i32>().unwrap())
            .collect();

        if is_safe(parts.iter()) {
            safe_count += 1;
        }
    }
    println!("Safe count: {}", safe_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_all_ascending() {
        let values = [1, 2, 3, 4, 5];
        let result = is_safe(values.iter());
        assert!(result)
    }

    #[test]
    fn test_safe_all_descending() {
        let values = [5, 4, 3, 2, 1];
        let result = is_safe(values.iter());
        assert!(result)
    }

    #[test]
    fn test_safe_one_wrong() {
        let values = [5, 6, 3, 2, 1];
        let result = is_safe(values.iter());
        assert!(result)
    }

    #[test]
    fn test_not_safe_two_wrong() {
        let values = [5, 6, 3, 7, 1];
        let result = is_safe(values.iter());
        assert!(!result)
    }
}

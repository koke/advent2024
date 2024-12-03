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

struct State {
    direction: i32,
    safe_count: i32,
}
pub fn second(path: String) {
    println!("Reading file {}", path);
    let file = match File::open(Path::new(path.as_str())) {
        Err(why) => panic!("Couldn't open {}: {}", path, why),
        Ok(file) => file,
    };
    let reader = io::BufReader::new(file);

    // let initial = State {
    //     direction: 0,
    //     safe_count: 0,
    // };
    // reader.lines().fold(initial, |state, line| {});

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
        let mut bad_levels = 0;

        if difference.abs() < 1 || difference.abs() > 3 {
            bad_levels = 1;
        }

        current = next;
        for mut next in parts {
            let difference = next - current;
            if difference.abs() < 1 || difference.abs() > 3 || difference.signum() != direction {
                if bad_levels == 0 {
                    bad_levels = 1;
                    next = current;
                } else {
                    safe = 0;
                    break;
                }
            }
            current = next;
        }
        safe_count += safe;
    }
    println!("Safe count: {}", safe_count);
}

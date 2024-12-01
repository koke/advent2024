use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn day1_1(path: &str) {
    println!("Reading file {}", path);
    // Open the file
    let file = match File::open(Path::new(path)) {
        Err(why) => panic!("Couldn't open {}: {}", path, why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file);

    let mut v_a: Vec<i32> = Vec::new();
    let mut v_b: Vec<i32> = Vec::new();

    for line in reader.lines() {
        // Parse two integers separted by a space
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        v_a.push(a);
        v_b.push(b);
    }

    v_a.sort();
    v_b.sort();

    let mut diff_sum = 0;

    for (a, b) in v_a.iter().zip(v_b.iter()) {
        // Calculate the absolute difference between the two numbers
        let diff = (a - b).abs();
        println!("a: {}, b: {}, diff: {}", a, b, diff);
        diff_sum += diff;
    }
    println!("Sum of differences: {}", diff_sum);
}

fn day1_2(path: &str) {
    println!("Reading file {}", path);
    // Open the file
    let file = match File::open(Path::new(path)) {
        Err(why) => panic!("Couldn't open {}: {}", path, why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file);

    let mut v_a: Vec<i32> = Vec::new();
    let mut v_b: Vec<i32> = Vec::new();

    for line in reader.lines() {
        // Parse two integers separted by a space
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        v_a.push(a);
        v_b.push(b);
    }

    let mut occurences = HashMap::new();

    for b in v_b.iter() {
        let count = occurences.entry(b).or_insert(0);
        *count += 1;
    }

    let mut similarity_sum = 0;
    for a in v_a.iter() {
        let similarity = a * occurences.get(a).unwrap_or(&0);
        println!("a: {}, similarity: {}", a, similarity);
        similarity_sum += similarity;
    }

    println!("Sum of similarities: {}", similarity_sum);
}

fn main() {
    day1_1("src/day1/example1.txt");
    day1_1("src/day1/input1.txt");
    day1_2("src/day1/example2.txt");
    day1_2("src/day1/input2.txt");
}

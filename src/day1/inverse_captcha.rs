#![allow(unused_must_use)]
use std::io::{self,BufRead};

// Convert the input stream into a vector of ints
fn collect_numbers() -> Vec<u32> {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut input);
    
    let numbers: Vec<u32> = input
                            .chars()
                            .filter_map(|c| c.to_digit(10))
                            .collect();
    return numbers;
}

// Sum the sequentially similar numbers
fn part1(numbers: Vec<u32>) -> u32 {
    // We'll start the loop by comparing against the last number
    let mut last = numbers.last().unwrap();
    let mut sum = 0;
    for n in &numbers {
        if n == last {
            sum += n;
        }
        last = &n;
    }
    return sum;
}

// Sum the circularly opposite numbers
fn part2(numbers: Vec<u32>) -> u32 {

    let half_length = numbers.len() / 2;

    let mut otherside = &numbers[half_length];
    let mut sum = 0;
    let mut pos = 0;
    for n in &numbers {
        if n == otherside {
            sum += n;
        }
        pos += 1;
        otherside = &numbers[(half_length + pos) % numbers.len()]
    }
    return sum;
}

fn main() {
    let numbers = collect_numbers();

    // Make sure the examples work first
    assert_eq!(3, part1(vec![1,1,2,2]));
    assert_eq!(4, part1(vec![1,1,1,1]));
    assert_eq!(0, part1(vec![1,2,3,4]));
    assert_eq!(9, part1(vec![9,1,2,1,2,1,2,9]));
    
    assert_eq!(6,  part2(vec![1,2,1,2]));
    assert_eq!(0,  part2(vec![1,2,2,1]));
    assert_eq!(4,  part2(vec![1,2,3,4,2,5]));
    assert_eq!(12, part2(vec![1,2,3,1,2,3]));
    assert_eq!(4,  part2(vec![1,2,1,3,1,4,1,5]));

    // Then process the challenge inputs
    println!("Part 1: {:?}", part1(numbers.clone()));
    println!("Part 2: {:?}", part2(numbers));
}

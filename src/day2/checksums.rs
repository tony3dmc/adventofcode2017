#![allow(unused_must_use)]
use std::io::{self,Read};

// Convert the input stream into vectors of ints
fn collect_numbers<'a>() -> Vec<Vec<u32>> { 
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input);
    
    let lines: Vec<String> = input.trim()
                                    .split("\n")
                                    .map(|s| s.to_string())
                                    .collect();
    let mut numbers: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        numbers.push(make_numbers(&line));
    }
    return numbers;
}

// Convert string into numbers
fn make_numbers(s: &String) -> Vec<u32> {
  return s.split("\t")
          .map(|s| s.to_string()
                    .parse::<u32>()
                    .unwrap())
          .collect();
}

// Work out the difference between highest and lowest
fn diff_extremes(mut ints: Vec<u32>) -> u32 {
    ints.sort();
    return ints.last().unwrap() - ints.first().unwrap();
}

// Find the clean division between the two factors
fn divide_factors(ints: Vec<u32>) -> u32 {
    let mut division = 0;
    for x in &ints {
        for y in &ints {
            if x % y == 0 && x != y {
                division = x / y;
            }
        }
    }
    return division;
}

// Calculate a checksum based on the differences between the extremes
fn part1(numbers: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for row in numbers {
        sum += diff_extremes(row.to_vec());
    }
    return sum;
}

// Calculate a checksum based on the division of the factors
fn part2(numbers: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for row in numbers {
        sum += divide_factors(row.to_vec());
    }
    return sum;
}

fn main() {

    // Make sure the examples work first
    assert_eq!(18, part1(&vec![
        vec![5, 1, 9, 5],
        vec![7, 5, 3   ],
        vec![2, 4, 6, 8]
    ]));
    assert_eq!(9, part2(&vec![
        vec![5, 9, 2, 8],
        vec![9, 4, 7, 3],
        vec![3, 8, 6, 5]
    ]));
    
    let numbers = &collect_numbers();

    // Then process the challenge inputs
    println!("Part 1: {:?}", part1(numbers));
    println!("Part 2: {:?}", part2(numbers));
}

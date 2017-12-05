#![allow(unused_must_use)]
use std::io::{self,Read};

// Convert the input stream into a vector of signed ints
fn collect_jumps() -> Vec<i32> { 
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input);
    
    let jumps: Vec<i32> = input.trim()
                               .split("\n")
                               .map(|s| s.to_string()
                                         .parse::<i32>()
                                         .unwrap())
                               .collect();
    return jumps;
}

fn modify_jump(jump: i32) -> i32 {
    return jump + 1;
}

fn modify_jump_more(jump: i32) -> i32 {
    if jump >= 3 {
        return jump - 1;
    } else {
        return jump + 1;
    }
}

// Jump the instructions back and forth
fn part1(mut jumps: Vec<i32>) -> u32 {
    let mut escaped = false;
    let mut count = 0;
    let mut pos = 0;

    while !escaped {
        let jump = jumps[pos as usize];

        jumps[pos as usize] = modify_jump(jump);

        pos += jump;

        if pos >= jumps.len() as i32 || pos < 0 {
            escaped = true;
        }
        count += 1;
    }

    return count;
}

// Jump the instructions back and forth in a wild way
fn part2(mut jumps: Vec<i32>) -> u32 {
    let mut escaped = false;
    let mut count = 0;
    let mut pos = 0;

    while !escaped {
        let jump = jumps[pos as usize];

        jumps[pos as usize] = modify_jump_more(jump);

        pos += jump;

        if pos >= jumps.len() as i32 || pos < 0 {
            escaped = true;
        }
        count += 1;
    }

    return count;
}

fn main() {
    // Make sure the examples work first
    assert_eq!(5,  part1(vec![0, 3, 0, 1, -3].clone()));
    assert_eq!(10, part2(vec![0, 3, 0, 1, -3].clone()));

    let jumps = &collect_jumps();

    // Then process the challenge inputs
    println!("Part 1: {:?}", part1(jumps.clone()));
    println!("Part 2: {:?}", part2(jumps.clone()));
}

#![allow(unused_must_use)]
use std::io::{self,Read};
use std::collections::HashMap;

// Convert the input stream into a vector of unsigned ints
fn collect_banks() -> Vec<u32> { 
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input);
    
    input.trim()
         .split(" ")
         .map(|s| s.to_string()
                   .parse::<u32>()
                   .unwrap())
         .collect()
}

fn highest_bank(banks: &Vec<u32>) -> usize {
    let mut highest = (0, 0);
    for i in 0..banks.len() {
        if banks[i] > highest.1 {
            highest = (i, banks[i]);
        }
    }
    highest.0
}

fn redistribute_memory(banks: &mut Vec<u32>) {
    let mut pos = highest_bank(banks);
    println!("Choosing bank {:?}", pos);
    let mut memory = banks[pos];
    banks[pos] = 0;
    while memory > 0 {
        pos = (pos + 1) % banks.len();
        banks[pos] += 1;
        memory -= 1;
    }
}

// How long does it take until we see the same bank config again?
fn part1(memory: &Vec<u32>) -> u32 {
    let mut banks = memory.clone();

    let mut history = HashMap::new();
    history.insert(banks.clone(), 0);
    println!("{:?}", banks);


    let mut count = 0;
    loop {
        count += 1;

        redistribute_memory(&mut banks);
        let history_item = banks.clone();
        println!("{:?}", history_item);

        if history.contains_key(&history_item) {
            return count;
        }

        history.insert(history_item, 0);
    }
}

// How big is the loop after finding the collision?
fn part2(memory: &Vec<u32>) -> u32 {
    let mut banks = memory.clone();

    let mut history = HashMap::new();
    history.insert(banks.clone(), 0);
    println!("{:?}", banks);


    let mut count = 0;
    let mut other = false;
    loop {
        count += 1;

        redistribute_memory(&mut banks);
        let history_item = banks.clone();
        println!("{:?}", history_item);

        if history.contains_key(&history_item) {
            if other {
                return count;
            }
            count = 0;
            history.clear();
            other = true;
        }

        history.insert(history_item, 0);
    }
}

fn main() {
    // Make sure the examples work first
    assert_eq!(5,  part1(&vec![0, 2, 7, 0]));
    assert_eq!(4,  part2(&vec![0, 2, 7, 0]));

    let banks = &collect_banks();

    // Then process the challenge inputs
    println!("Part 1: {:?}", part1(&banks));
    println!("Part 2: {:?}", part2(&banks));
}

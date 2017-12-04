#![allow(unused_must_use)]
use std::io::{self,Read};

// Convert the input stream into a vector of strings
fn collect_phrases<'a>() -> Vec<String> { 
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input);
    
    let phrases: Vec<String> = input.trim()
                                    .split("\n")
                                    .map(|s| s.to_string())
                                    .collect();
    return phrases;
}

// Sort the letters within a string 
fn sort_letters(s: String) -> String {
  let mut chars = s.chars().collect::<Vec<char>>();
  chars.sort();
  return chars.into_iter().collect::<String>();
}

// Count the number of phrases with no duplicate words
fn part1(phrases: &Vec<String>) -> u32 {
    let mut sum = 0;
    for phrase in phrases {
        let original_words = phrase.split_whitespace();

        let mut dedup_words = original_words.clone()
                                            .map(|s| s.to_string())
                                            .collect::<Vec<String>>();
        dedup_words.sort();
        dedup_words.dedup();

        if original_words.count() == dedup_words.len() {
            sum += 1;
        }
    }
    return sum;
}

// Count the number of phrass with no duplicate anigram words
fn part2(phrases: &Vec<String>) -> u32 {
    let mut sum = 0;

    for phrase in phrases {
        let original_words = phrase.split_whitespace()
                                .map(|s| sort_letters(String::from(s)))
                                .collect::<Vec<String>>();

        let mut dedup_words = original_words.clone()
                                            .iter()
                                            .map(|s| s.to_string())
                                            .collect::<Vec<String>>();
        dedup_words.sort();
        dedup_words.dedup();

        if original_words.len() == dedup_words.len() {
            sum += 1;
        }
    }
    return sum;
}

fn main() {
    // Make sure the examples work first
    assert_eq!(1, part1(&vec![String::from("aa bb cc dd ee")]));
    assert_eq!(0, part1(&vec![String::from("aa bb cc dd aa")]));
    assert_eq!(1, part1(&vec![String::from("aa bb cc dd aaa")]));
    
    assert_eq!(1, part2(&vec![String::from("abcde fghij")]));
    assert_eq!(0, part2(&vec![String::from("abcde xyz ecdab")]));
    assert_eq!(1, part2(&vec![String::from("a ab abc abd abf abj")]));
    assert_eq!(1, part2(&vec![String::from("iiii oiii ooii oooi oooo")]));
    assert_eq!(0, part2(&vec![String::from("oiii ioii iioi iiio")]));

    let phrases = &collect_phrases();

    // Then process the challenge inputs
    println!("Part 1: {:?}", part1(phrases));
    println!("Part 2: {:?}", part2(phrases));
}

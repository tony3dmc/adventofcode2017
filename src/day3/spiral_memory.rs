#![allow(unused_must_use)]
use std::io::{self,Read};

// Convert the input into a number
fn collect_input() -> u32 { 
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input);
    
    return input.trim().to_string().parse::<u32>().unwrap();
}

#[derive(Debug)]
struct Thing {
    position: (i32, i32),
    direction: u32,
    edge_length: u32,
    current_edge_position: u32,
}

fn part1(target: u32) -> i32 {
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut thing = Thing {
        position: (0, 0),
        direction: 0,
        edge_length: 1,
        current_edge_position: 0,
    };

    let mut current_direction = directions[thing.direction as usize];
    let mut time_to_change = true;

    for _ in 2..(target + 1) {
        if thing.current_edge_position >= thing.edge_length {
            thing.direction = (thing.direction + 1) % directions.len() as u32;
            current_direction = directions[thing.direction as usize];
            thing.current_edge_position = 0;
            time_to_change = !time_to_change;
            if time_to_change {
                thing.edge_length += 1;
            }
        }

        thing.position.0 += current_direction.0;
        thing.position.1 += current_direction.1;
        
        thing.current_edge_position += 1;
    }

    return thing.position.0.abs() + thing.position.1.abs();
}


fn main() {
    // Make sure the examples work first
    assert_eq!(0, part1(1));
    assert_eq!(3, part1(12));
    assert_eq!(2, part1(23));
    assert_eq!(31, part1(1024));
    
    let target = collect_input();

    // Then process the challenge inputs
    println!("Part 1: {:?}", part1(target));
}

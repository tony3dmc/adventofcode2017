#![allow(unused_must_use)]
use std::io::{self,Read};
use std::collections::HashMap;

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
    edge_pos: u32,
}

fn part1(target: u32) -> i32 {
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut thing = Thing {
        position: (0, 0),
        direction: 0,
        edge_length: 1,
        edge_pos: 0,
    };

    let mut current_direction = directions[thing.direction as usize];
    let mut time_to_change = true;

    for _ in 2..(target + 1) {
        if thing.edge_pos >= thing.edge_length {
            thing.direction = (thing.direction + 1) % directions.len() as u32;
            current_direction = directions[thing.direction as usize];
            thing.edge_pos = 0;
            time_to_change = !time_to_change;
            if time_to_change {
                thing.edge_length += 1;
            }
        }

        thing.position.0 += current_direction.0;
        thing.position.1 += current_direction.1;
        
        thing.edge_pos += 1;
    }

    return thing.position.0.abs() + thing.position.1.abs();
}

fn sum_tuples(a: (i32, i32), b: &(i32, i32)) -> (i32, i32) {
  (a.0 + b.0, a.1 + b.1)
}

fn part2(target: u32) -> u32 {
    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    let all_directions = vec![
        (-1,  1), (0, 1),  (1,  1),
        (-1,  0),          (1,  0),
        (-1, -1), (0, -1), (1, -1),
    ];

    let mut thing = Thing {
        position: (0, 0),
        direction: 0,
        edge_length: 1,
        edge_pos: 0,
    };

    let mut current_direction = directions[thing.direction as usize];
    let mut time_to_change = true;
    let mut value = 1;

    let mut map = HashMap::new();
    map.insert(thing.position, value);

    // let mut step = 0;

    while value < target {
        // step += 1;

        if thing.edge_pos >= thing.edge_length {
            thing.direction = (thing.direction + 1) % directions.len() as u32;
            current_direction = directions[thing.direction as usize];
            thing.edge_pos = 0;
            time_to_change = !time_to_change;
            if time_to_change {
                thing.edge_length += 1;
            }
        }

        thing.position.0 += current_direction.0;
        thing.position.1 += current_direction.1;

        // println!("Iteration {:?} at position {:?}:", step, thing.position);

        value = 0;
        for direction in all_directions.iter() {
            let that_position = sum_tuples(thing.position, direction);
            // println!("  Looking in direction {:?} at position {:?}", direction, that_position);
            if map.contains_key(&that_position) {
                // println!("    There's a value in direction {:?}, being position {:?} of {:?}", direction, that_position, map[&that_position]);
                value += map[&that_position];
                // println!("      Value is now {:?}", value);
            }
        }

        map.insert(thing.position, value);
        
        thing.edge_pos += 1;
    }

    return value;
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

    println!("Part 2: {:?}", part2(target));
}

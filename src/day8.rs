use std::collections::HashMap;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Coord {
    x: u32,
    y: u32,
    z: u32
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Input {
    coords : Vec<Coord>
}

#[aoc_generator(day8)]
pub fn input_generator(raw_input: &str) -> Input {
    let coords = raw_input.lines().map(|l| {
        let parts : Vec<&str> = l.split(',').collect();
        assert!(parts.len() == 3);
        Coord{ x: parts[0].parse().unwrap(), y: parts[1].parse().unwrap(), z: parts[2].parse().unwrap() }
    }).collect();
    Input{ coords }
}

fn get_distance(a: &Coord, b: &Coord) -> f32 {
    let dx = (a.x as i32 - b.x as i32) as f32;
    let dy = (a.y as i32 - b.y as i32) as f32;
    let dz = (a.z as i32 - b.z as i32) as f32;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

#[aoc(day8, part1)]
pub fn part1(input: &Input) -> usize {
    let mut connections = Vec::new();
    for (i, ic) in input.coords.iter().enumerate() {
        for j in (i+1)..input.coords.len() {
            if i != j {
                connections.push((i, j, get_distance(ic, &input.coords[j])));
            }
        }
    }
    connections.sort_by(|a, b| { 
        if a.2 < b.2 { std::cmp::Ordering::Less }
        else if a.2 == b.2 { std::cmp::Ordering::Equal }
        else { std::cmp::Ordering::Greater }
    });
    
    let mut box_idx_to_circuit_idx = Vec::new();
    let mut circuit_idx_to_boxes_map = HashMap::new();
    for i in 0..input.coords.len() {
        let circuit_idx = box_idx_to_circuit_idx.len();
        box_idx_to_circuit_idx.push(circuit_idx);
        circuit_idx_to_boxes_map.insert(circuit_idx, vec!(i));
    }

    let num_wires = 1000;
    for i in 0..num_wires {
        let box0 = connections[i].0;
        let box1 = connections[i].1;
        let circuit0 = box_idx_to_circuit_idx[box0];
        let circuit1 = box_idx_to_circuit_idx[box1];

        if circuit0 != circuit1 {
            for b in circuit_idx_to_boxes_map.get(&circuit0).unwrap().clone() {
                box_idx_to_circuit_idx[b] = circuit1;
                circuit_idx_to_boxes_map.get_mut(&circuit1).unwrap().push(b);
            }
            circuit_idx_to_boxes_map.get_mut(&circuit0).unwrap().clear();
        }
    }   
    let mut circuit_sizes = circuit_idx_to_boxes_map.iter().map(|c| c.1.len()).sorted().rev();
    circuit_sizes.next().unwrap() * circuit_sizes.next().unwrap() * circuit_sizes.next().unwrap()
}

#[aoc(day8, part2)]
pub fn part2(input: &Input) -> u64 {
    let mut connections = Vec::new();
    for (i, ic) in input.coords.iter().enumerate() {
        for j in (i+1)..input.coords.len() {
            if i != j {
                connections.push((i, j, get_distance(ic, &input.coords[j])));
            }
        }
    }
    connections.sort_by(|a, b| { 
        if a.2 < b.2 { std::cmp::Ordering::Less }
        else if a.2 == b.2 { std::cmp::Ordering::Equal }
        else { std::cmp::Ordering::Greater }
    });
    
    let mut box_idx_to_circuit_idx = Vec::new();
    let mut circuit_idx_to_boxes_map = HashMap::new();
    for i in 0..input.coords.len() {
        let circuit_idx = box_idx_to_circuit_idx.len();
        box_idx_to_circuit_idx.push(circuit_idx);
        circuit_idx_to_boxes_map.insert(circuit_idx, vec!(i));
    }

    let mut i = 0;
    loop {
        let box0 = connections[i].0;
        let box1 = connections[i].1;
        let circuit0 = box_idx_to_circuit_idx[box0];
        let circuit1 = box_idx_to_circuit_idx[box1];

        if circuit0 != circuit1 {
            for b in circuit_idx_to_boxes_map.get(&circuit0).unwrap().clone() {
                box_idx_to_circuit_idx[b] = circuit1;
                circuit_idx_to_boxes_map.get_mut(&circuit1).unwrap().push(b);
            }
            circuit_idx_to_boxes_map.get_mut(&circuit0).unwrap().clear();

            if circuit_idx_to_boxes_map.get(&circuit1).unwrap().len() == input.coords.len() {
                break;
            }
        }
        i += 1;
    }
    input.coords[connections[i].0].x as u64 * input.coords[connections[i].1].x as u64
}

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Coord {
    x: i32,
    y: i32,
}

type CoordSet = std::collections::HashSet::<Coord>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> CoordSet {
    let mut coords = CoordSet::new();
    for line in input.lines().enumerate() {
        for c in line.1.chars().enumerate() {
            if c.1 == '@' {
                coords.insert(Coord{ x: c.0 as i32, y: line.0 as i32 });
            }
        }
    }
    coords
}

fn forklift_has_access(coords: &CoordSet, coord: &Coord) -> bool {
    let deltas = [
        Coord{ x: -1, y: -1 },
        Coord{ x: 0, y: -1 },
        Coord{ x: 1, y: -1 },
        Coord{ x: -1, y: 0 },
        Coord{ x: 1, y: 0 },
        Coord{ x: -1, y: 1 },
        Coord{ x: 0, y: 1 },
        Coord{ x: 1, y: 1 },
    ];
    let mut num_neighbors = 0;
    for delta in deltas.iter() {
        let neighbor = Coord{ x: coord.x + delta.x, y: coord.y + delta.y };
        if coords.contains(&neighbor) {
            num_neighbors += 1;
            if num_neighbors >= 4 {
                return false;
            }
        }
    }
    true
}

#[aoc(day4, part1)]
pub fn part1(coords: &CoordSet) -> u32 {
    coords.iter().map(|c| match forklift_has_access(coords, c) {
        true => 1,
        false => 0,
    }).sum()
}

#[aoc(day4, part2)]
pub fn part2(in_coords: &CoordSet) -> u32 {
    let mut mut_coords = in_coords.clone();
    let mut num_total_removed = 0;
    loop {
        let to_remove : Vec<Coord> = mut_coords.iter().filter(|c| forklift_has_access(&mut_coords, c)).cloned().collect();
        if to_remove.is_empty() {
            break;
        }
        else {
            num_total_removed += to_remove.len() as u32;
            for coord in to_remove.iter() {
                mut_coords.remove(coord);
            }
        }
    }
    num_total_removed
}
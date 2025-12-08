use std::collections::HashSet;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Coord {
    x : i32,
    y : i32
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Input {
    start : Coord,
    splitters : HashSet<Coord>,
    max_x : i32,
    max_y : i32
}

#[aoc_generator(day7)]
pub fn input_generator(raw_input: &str) -> Input {
    let mut splitters = HashSet::new();
    let mut start = Coord{ x:0, y:0 };
    let mut max_y = 0;
    let mut max_x = 0;
    for (y, line) in raw_input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coord{x:x as i32, y:y as i32};
            match c {
                'S' => start = coord,
                '^' => { splitters.insert(coord); },
                '.' => (),
                _ => panic!()
            }
            max_x = max_x.max(x);
        }
        max_y = max_y.max(y);
    }

    Input{ start, splitters, max_x:max_x as i32, max_y:max_y as i32 }
}

#[aoc(day7, part1)]
pub fn part1(input: &Input) -> i32 {
    let mut num_splits = 0;
    let mut beams = Vec::<i32>::new();
    beams.push(input.start.x);    
    for y in (input.start.y + 1)..=input.max_y {
        let mut new_beams = HashSet::new();
        for x in beams {
            if input.splitters.contains(&Coord{x, y}) {
                new_beams.insert(x - 1);
                new_beams.insert(x + 1);
                num_splits += 1;
            }
            else {
                new_beams.insert(x);
            }
        }
        beams = new_beams.into_iter().collect_vec();
    }
    num_splits
}

#[aoc(day7, part2)]
pub fn part2(input: &Input) -> u64 {
    let mut timelines = Vec::new();
    for _ in 0..=input.max_x {
        timelines.push(1);
    }

    for y in ((input.start.y + 1)..=input.max_y).rev() {
        let prev_timelines = timelines.clone();
        for x in 0..=input.max_x {
            if input.splitters.contains(&Coord{x, y}) {
                let mut split = 0;
                if x > 0 {
                    split += prev_timelines[(x - 1) as usize];
                }
                if x < input.max_x {
                    split += prev_timelines[(x + 1) as usize];
                }
                timelines[x as usize] = split;
            }
        }
    }

    timelines[input.start.x as usize]
}

// #[aoc(day7, part2)]
// pub fn part2(input: &Input) -> u64 {

//     fn get_num_timelines(input: &Input, cache: &mut HashMap<Coord, u64>, at : &Coord, curr_num: u64) -> u64 {
//         if at.x < 0 || at.x > input.max_x {
//             return 0;
//         }
        
//         let next = Coord{x:at.x, y: at.y + 1};
//         if next.y > input.max_y {
//             return curr_num;
//         }

//         match cache.get(&next) {
//             Some(n) => *n,
//             None => {
//                 let num_next = match input.splitters.contains(&next) {
//                     true => {
//                         get_num_timelines(input, cache, &Coord{x:next.x - 1, y:next.y}, curr_num) +
//                         get_num_timelines(input, cache, &Coord{x:next.x + 1, y:next.y}, curr_num)
//                     },
//                     false => {
//                         get_num_timelines(input, cache, &next, curr_num)
//                     }
//                 };
//                 cache.insert(next, num_next);
//                 num_next
//             }
//         }        
//     }

//     let mut num_timelines_cache = HashMap::<Coord, u64>::new();
//     get_num_timelines(input, &mut num_timelines_cache, &input.start, 1)
// }
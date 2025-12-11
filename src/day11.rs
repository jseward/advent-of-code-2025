use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Input {
    devices : HashMap<String, Vec<String>>
}

#[aoc_generator(day11)]
pub fn input_generator(raw_input: &str) -> Input {
    Input{ devices :
        raw_input.lines().map(|line| {
            line.split_once(':').map(|parts| {
                (
                    parts.0.trim().to_string(), 
                    parts.1.trim().split(' ').map(|s| s.to_string()).collect()
                )
            }).unwrap()
        }).collect::<HashMap<String, Vec<String>>>()
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &Input) -> u32 {
    let mut num_out_paths = 0;
    let mut paths = VecDeque::<Vec<String>>::new();
    paths.push_back(vec!("you".to_string()));
    while !paths.is_empty() {
        let path = paths.pop_front().unwrap();
        let curr = path.last().unwrap();
        for next in input.devices.get(curr).unwrap() {
            if next == "out" {
                num_out_paths += 1;
            }
            else {
                if !path.contains(next) {
                    let mut new_path = path.clone();
                    new_path.push(next.clone());
                    paths.push_back(new_path);
                }
            }
        }
    }
    num_out_paths
}

fn get_num_out_paths2(input: &Input, cache : &mut HashMap<(String, bool, bool), u64>, path : Vec<String>) -> u64
{
    let curr = path.last().unwrap();
    let has_fft = path.contains(&"fft".to_string());
    let has_dac = path.contains(&"dac".to_string());
    match cache.get(&(curr.clone(), has_fft, has_dac)) {
        Some(n) => *n,
        None => {
            let mut acc = 0;
            for next in input.devices.get(curr).unwrap() {
                if next == "out" {                    
                    if has_fft && has_dac {
                        acc += 1;
                    }
                }
                else {
                    if !path.contains(next) {
                        let mut new_path = path.clone();
                        new_path.push(next.clone());
                        acc += get_num_out_paths2(input, cache, new_path);
                    }
                }
            }
            cache.insert((curr.clone(), has_fft, has_dac), acc);                
            acc
        }
    }
}

#[aoc(day11, part2)]
pub fn part2(input: &Input) -> u64 {

    let mut cache = HashMap::new();
    get_num_out_paths2(input, &mut cache, vec!("svr".to_string()))
}

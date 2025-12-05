use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Input {
    fresh_ids : Vec<(u64, u64)>,
    available_ids : Vec<u64>
}

#[aoc_generator(day5)]
pub fn input_generator(raw_input: &str) -> Input {
    let mut fresh_ids = Vec::new();
    let mut available_ids = Vec::new();
    let mut is_fresh = true;
    for line in raw_input.lines() {
        if line.trim().is_empty() {
            is_fresh = false;
        }
        else {
            match is_fresh {
                true => {
                    let parts = line.split_once('-').unwrap();
                    fresh_ids.push(( parts.0.parse::<u64>().unwrap(), parts.1.parse::<u64>().unwrap() ));
                }
                false => {
                    available_ids.push(line.parse::<u64>().unwrap());
                }
            }
        }
    }
    Input{ fresh_ids, available_ids }
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> u32 {
    let mut num_fresh_available = 0;
    for available_id in input.available_ids.iter() {
        if input.fresh_ids.iter().any(|fresh_id| {
            *available_id >= fresh_id.0 && *available_id <= fresh_id.1 } ) {
                num_fresh_available += 1;
        }
    }
    num_fresh_available
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> u64 {
    let mut in_ranges = input.fresh_ids.clone();
    let mut out_ranges : Vec<(u64, u64)> = Vec::new();

    while !in_ranges.is_empty() {
        let in_r = in_ranges.pop().unwrap();
        assert!(in_r.0 <= in_r.1);

        let mut overlapped = false;
        for out_r in out_ranges.iter_mut() {
            if in_r.0 < out_r.0 && in_r.1 > out_r.1 {
                in_ranges.push( (in_r.0, out_r.0 - 1) );
                in_ranges.push( (out_r.1 + 1, in_r.1) );
                overlapped = true;
                break;
            }

            if in_r.0 < out_r.0 && in_r.1 >= out_r.0 {
                in_ranges.push( (in_r.0, out_r.0 - 1) );
                overlapped = true;
                break;
            }

            if in_r.0 <= out_r.1 && in_r.1 > out_r.1 {
                in_ranges.push( (out_r.1 + 1, in_r.1) );
                overlapped = true;
                break;
            }

            if in_r.0 >= out_r.0 && in_r.1 <= out_r.1 {
                overlapped = true;
                break;
            }
        }

        if !overlapped {
            out_ranges.push(in_r);
        }              
    }

    out_ranges.iter().map(|r| r.1 - r.0 + 1).sum::<u64>()
}

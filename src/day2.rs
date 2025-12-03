use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;


#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(String, String)> {
    input.split(',').map(|r| { match r.split_once('-').unwrap() {
        (a, b) => (a.to_string(), b.to_string())
    }
    }).collect()
}

fn is_invalid_1(v: &str) -> bool {
    let vb = v.as_bytes();
    match vb.len() % 2 == 0 {
        true => {
            for i in 0..(vb.len() / 2) {
                if vb[i] != vb[vb.len() / 2 + i] {
                    return false;
                }
            }
            true
        },
        false => false
    }
}

// an ID is invalid if it is made only of some sequence of digits repeated at least twice
fn is_invalid_2(v: &str) -> bool {    
    let vb = v.as_bytes();
    for seq_len in 1..=(vb.len() / 2) {
        if vb.len() % seq_len == 0 {
            let num_seq = vb.len() / seq_len;
            let mut all_match = true;
            for seq_i in 1..num_seq {
                for i in 0..seq_len {
                    if vb[i] != vb[(seq_i * seq_len) + i] {
                        all_match = false;
                    }
                }
            }            
            if all_match {
                return true;
            }
        }
    }
    false
}

fn sum_invalid(is_invalid_func: fn(v: &str) -> bool, a: &String, b: &String) -> u64 {
    let mut sum: u64 = 0;
    let a_v = a.parse::<u64>().expect(&format!("failed to parse {a}"));
    let b_v = b.parse::<u64>().expect(&format!("failed to parse {b}"));
    for v in a_v..=b_v {
        if is_invalid_func(v.to_string().as_str()) {
            sum += v;
        }
    }
    sum
}

#[aoc(day2, part1)]
pub fn part1<'a>(input: &Vec<(String, String)>) -> u64 {
    input.into_iter().map(|(a, b)| {
        sum_invalid(is_invalid_1, a, b)
    }).sum()
}

#[aoc(day2, part2)]
pub fn part2<'a>(input: &Vec<(String, String)>) -> u64 {
    input.into_iter().map(|(a, b)| {
        sum_invalid(is_invalid_2, a, b)
    }).sum()
}
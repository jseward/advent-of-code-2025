use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;


#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|bank| 
            bank
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect())
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(banks: &Vec<Vec<u32>>) -> u32 {
    banks
        .iter()
        .map(|bank| {
            let mut high_0 = 0;
            let mut high_1 = 0;
            for i in 0..bank.len() {
                if bank[i] > high_0 && i < bank.len() - 1 {
                    high_0 = bank[i];
                    high_1 = 0;
                }
                else {
                    high_1 = high_1.max(bank[i]);
                }
            }
            (high_0 * 10) + high_1
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(banks: &Vec<Vec<u32>>) -> u64 {
    banks
        .iter()
        .map(|bank| {
            let mut hs = [0; 12];
            let mut begin = 0;
            for hs_idx in 0..hs.len() {
                let num_remaining_hs = hs.len() - hs_idx - 1;
                let end = bank.len() - num_remaining_hs;
                for i in begin..end {
                    if hs[hs_idx] < bank[i] {
                        hs[hs_idx] = bank[i];
                        begin = i + 1;
                    }
                }
            }
            let mut res = 0;
            for i in 0..hs.len() {
                res += hs[i] as u64 * 10u64.pow((hs.len() - i - 1) as u32);
            }
            res
        })
        .sum()
}
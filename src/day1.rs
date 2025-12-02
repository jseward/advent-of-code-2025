use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

pub enum Rotation {
    L(i32),
    R(i32)
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            match chars.next().unwrap() {
                'L' => Rotation::L(chars.as_str().parse().unwrap()),
                'R' => Rotation::R(chars.as_str().parse().unwrap()),
                _ => panic!("Invalid input")}
            }
        ).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<Rotation>) -> i32 {
    let mut c = 0;
    let mut pos = 50;
    for r in input {
        match r {
            Rotation::L(n) => {
                pos -= n;
                while pos < 0 {
                    pos += 100;
                }
            },
            Rotation::R(n) => {
                pos += n;
                while pos >= 100 {
                    pos -= 100;
                }
            }
        }
        if pos == 0 {
            c += 1;
        }
    }
    c
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<Rotation>) -> i32 {
    let mut c = 0;
    let mut pos = 50;
    for r in input {
        match r {
            Rotation::L(n) => {
                let mut count_click = pos != 0;
                pos -= n;
                while pos < 0 {
                    pos += 100;
                    if count_click {
                        c += 1;
                    }
                    count_click = true;
                }
                if pos == 0 {
                    c += 1;
                }
            },
            Rotation::R(n) => {
                pos += n;
                while pos >= 100 {
                    pos -= 100;
                    c += 1;
                }
            }
        }
    }
    c
}
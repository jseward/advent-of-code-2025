use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Coord {
    x: u64,
    y: u64
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Input {
    coords : Vec<Coord>,
    min_x : u64,
    max_x : u64,
    min_y : u64,
    max_y : u64
}

#[aoc_generator(day9)]
pub fn input_generator(raw_input: &str) -> Input {
    let mut min_x = 0;
    let mut min_y = 0;    
    let mut max_x = 0;
    let mut max_y = 0;
    let coords = raw_input.lines().map(|l| {
        let parts : Vec<&str> = l.split(',').collect();
        assert!(parts.len() == 2);
        let x = parts[0].parse().unwrap();
        let y = parts[1].parse().unwrap();
        min_x = min_x.min(x);
        min_y = min_y.min(y);        
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        Coord{ x, y }
    }).collect();
    Input{ coords, min_x, max_x, min_y, max_y }
}

#[aoc(day9, part1)]
pub fn part1(input: &Input) -> u64 {
    let mut max_a = 0;
    for (i, c) in input.coords.iter().enumerate() {
        for j in i + 1..input.coords.len() {
            let x0 = c.x;
            let x1 = input.coords[j].x;            
            let dx = x0.max(x1) - x0.min(x1) + 1;
            let y0 = c.y;
            let y1 = input.coords[j].y;            
            let dy = y0.max(y1) - y0.min(y1) + 1;
            let a = dx * dy;
            max_a = max_a.max(a);
        }
    }
    max_a
}

#[aoc(day9, part2)]
pub fn part2(input: &Input) -> u64 {

    let is_rect_solid = |min_x: u64, max_x: u64, min_y: u64, max_y: u64| {

        let is_line_segment_intersecting = |a: &Coord, b: &Coord| {
            for x in a.x.min(b.x)..=a.x.max(b.x) {
                for y in a.y.min(b.y)..=a.y.max(b.y) {
                    if x > min_x && x < max_x && y > min_y && y < max_y {
                        return true;
                    }
                }
            }
            false
        };

        let mut coords_iter = input.coords.iter();
        let mut curr = coords_iter.next().unwrap();
        let first = curr;
        while let Some(next) = coords_iter.next() {
            if is_line_segment_intersecting(curr, next) {
                return false;
            }
            curr = next;
        }
        
        if is_line_segment_intersecting(curr, first) {
            return false;
        }

        true
    };

    let mut max_a = 0;
    for (i, c0) in input.coords.iter().enumerate() {
        for j in i + 1..input.coords.len() {
            let c1 = &input.coords[j];
            let x0 = c0.x;
            let x1 = c1.x;            
            let y0 = c0.y;
            let y1 = c1.y;    
            let dx = x0.max(x1) - x0.min(x1) + 1;
            let dy = y0.max(y1) - y0.min(y1) + 1;
            let a = dx * dy;
            if a > max_a && is_rect_solid(x0.min(x1), x0.max(x1), y0.min(y1), y0.max(y1)) {
                max_a = a;
            }
        }
    }
    max_a
}

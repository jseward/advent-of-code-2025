use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Op {
    Add,
    Multiply,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Column {
    values : Vec<String>,
    op : Op
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Input {
    columns : Vec<Column>
}

#[aoc_generator(day6)]
pub fn input_generator(raw_input: &str) -> Input {
    let mut rows_of_spans : Vec<Vec<(usize, usize)>> = Vec::new();
    for line in raw_input.lines() {
        let mut spans = Vec::new();
        let mut chars = line.chars();
        let mut begin = None;
        let mut i = 0;
        loop {
            match chars.next() {
                Some(c) if c == ' ' => {
                    if let Some(b) = begin {
                        spans.push( (b, i - 1) );
                        begin = None;
                    }
                },
                Some(_) => {
                    if begin.is_none() {
                        begin = Some(i);
                    }
                },
                None => {
                    if let Some(b) = begin {
                        spans.push( (b, i - 1) );
                    }
                    break;
                }
            }
            i += 1;
        }
        rows_of_spans.push(spans);
    }

    let mut resolved_spans : Vec<(usize, usize)> = Vec::new();
    for col_idx in 0..rows_of_spans[0].len() {
        let mut span_begin = rows_of_spans[0][col_idx].0;
        let mut span_end = rows_of_spans[0][col_idx].1;
        for row_idx in 1..rows_of_spans.len() {
            span_begin = span_begin.min(rows_of_spans[row_idx][col_idx].0);
            span_end = span_end.max(rows_of_spans[row_idx][col_idx].1);
        }
        if !resolved_spans.is_empty() {
            assert!(span_begin == resolved_spans.last().unwrap().1 + 2);
        }
        resolved_spans.push( (span_begin, span_end) );
    }

    let mut columns : Vec<Column> = vec![
        Column { values: Vec::new(), op: Op::Add };
        resolved_spans.len()
    ];
    for line in raw_input.lines() {
        let is_ops = match line.trim().chars().next().unwrap() {
            '*' | '+' => true,
            _ => false
        };
        if is_ops {
            line
                .split_whitespace()
                .map(|op_str| match op_str {
                    "+" => Op::Add,
                    "*" => Op::Multiply,
                    _ => panic!(),
                })
                .enumerate()
                .for_each(|(i, op)| {
                    columns[i].op = op;
                });
        }
        else {
            let mut chars = line.chars();
            let mut char_idx = 0;
            for (i, span) in resolved_spans.iter().enumerate() {
                while char_idx < span.0 {
                    chars.next().unwrap();
                    char_idx += 1;
                }

                let mut span_string = String::new(); 
                while char_idx <= span.1 {
                    span_string.push(chars.next().unwrap());
                    char_idx += 1;
                }                

                columns[i].values.push(span_string);
            }
        }
    }
    Input{ columns }
}

#[aoc(day6, part1)]
pub fn part1(input: &Input) -> u64 {
    input.columns.iter().map(|column| {
        match column.op {
            Op::Add => column.values.iter().map(|v| v.trim().parse::<u64>().unwrap() ).sum::<u64>(),
            Op::Multiply => column.values.iter().map(|v| v.trim().parse::<u64>().unwrap() ).product::<u64>(),
        }
    }).sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &Input) -> u64 {
    let mut final_total = 0;
    for c in input.columns.iter() {
        let mut c_total = match c.op {
            Op::Add => 0,
            Op::Multiply => 1,
        };

        let max_val_len = c.values.iter().map(|v| v.len()).max().unwrap();

        let mut i = 0;
        loop {
            let mut vs = String::new();
            for cv in c.values.iter() {
                match cv.chars().nth(max_val_len - i) {
                    Some(ch) => vs.push(ch),
                    None => ()
                }
            }
            
            if vs.is_empty() {
                break;
            }

            match c.op {
                Op::Add => {
                    c_total += vs.trim().parse::<u64>().unwrap();
                }
                Op::Multiply => {
                    c_total *= vs.trim().parse::<u64>().unwrap();
                }
            }

            i += 1;
        }
        final_total += c_total;
    }
    final_total
}

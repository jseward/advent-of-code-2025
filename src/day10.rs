use std::collections::VecDeque;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Machine {
    start_lights : Vec<bool>,
    buttons : Vec<Vec<u32>>,
    joltage_levels : Vec<u32>
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Input {
    machines : Vec<Machine>
}

#[aoc_generator(day10)]
pub fn input_generator(raw_input: &str) -> Input {
    let mut machines = Vec::new();

    for line in raw_input.lines() {
        let mut start_lights = Vec::new();

        let split_0 = line.split_once(']').unwrap();
        for (i, c) in split_0.0.chars().enumerate() {
            if i > 0 {
                start_lights.push(match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!()
                })
            }
        }

        let mut buttons = Vec::new();
        let split_1 = split_0.1.split_once('{').unwrap();
        for button_str in split_1.0.split(' ') {
            let mut button = Vec::new();
            if !button_str.trim().is_empty() {
                let chars = button_str.chars().collect_vec();
                assert!(chars[0] == '(');
                assert!(chars[chars.len() - 1] == ')');
                for ns in chars[1..chars.len() - 1].iter().collect::<String>().split(',') {
                    button.push(ns.parse().unwrap());
                }
                buttons.push(button);
            }
        }

        let split_2 = split_1.1.split_once('}').unwrap();
        let joltage_levels : Vec<u32> = split_2.0.split(',').map(|s| s.parse().unwrap()).collect();

        machines.push(Machine{start_lights, buttons, joltage_levels}); 
    }
    Input { machines }
}


#[aoc(day10, part1)]
pub fn part1(input: &Input) -> u32 {

    #[derive(PartialEq, Eq, Hash, Debug, Clone)]
    struct MachineState {
        lights : Vec<bool>,
        num_pushes : u32
    }

    let get_fewest_pushes = |machine: &Machine| {
        let mut states = VecDeque::new();
        states.push_back(MachineState{ 
            lights: vec![false; machine.start_lights.len()],
            num_pushes : 0
        });
        loop {
            let state = states.pop_front().unwrap();
            for button in &machine.buttons {
                let mut new_state = state.clone();
                for light_idx in button{
                    new_state.lights[*light_idx as usize] = !new_state.lights[*light_idx as usize];
                }
                new_state.num_pushes += 1;
                if new_state.lights.eq(&machine.start_lights) {
                    return new_state.num_pushes;
                }
                states.push_back(new_state);
            }
        }
    };

    input.machines.iter().map(|m| get_fewest_pushes(m)).sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &Input) -> u32 {

    let get_fewest_pushes = |machine: &Machine| {
        let mut primary_jolt_idx : Option<usize> = None;
        let mut all_jolt_buttons : Vec<Vec<usize>> = Vec::new();
        for jolt_idx in 0..machine.joltage_levels.len() {
            let jolt_buttons : Vec<usize> = machine.buttons.iter().enumerate().filter_map(|(button_idx, button)| {
                match button.contains(&(jolt_idx as u32)) {
                    true => Some(button_idx),
                    false => None
                }
            }).collect();
            if primary_jolt_idx.is_none() || jolt_buttons.len() < all_jolt_buttons[primary_jolt_idx.unwrap()].len() {
                primary_jolt_idx = Some(jolt_idx);
            }
            all_jolt_buttons.push(jolt_buttons);
        }

        // todo

        0
    };

    input.machines.iter().map(|m| get_fewest_pushes(m)).sum()
}



// #[aoc(day10, part2)]
// pub fn part2(input: &Input) -> u32 {

//     #[derive(PartialEq, Eq, Hash, Debug, Clone)]
//     struct MachineState {
//         levels : Vec<u32>,
//         num_pushes : u32
//     }

//     let get_fewest_pushes_0 = |machine: &Machine, seed_button_idx: usize, seed_max_pushes_offset: u32| {

//         let seed_button = &machine.buttons[seed_button_idx as usize];

//         let max_pushes = seed_button
//             .iter()
//             .map(|level_idx| machine.joltage_levels[*level_idx as usize])
//             .min()
//             .unwrap();
//         if max_pushes <= seed_max_pushes_offset {
//             return None;
//         }

//         let mut seed_machine_state = MachineState{ 
//             levels: vec![0; machine.joltage_levels.len()],
//             num_pushes: 0
//         };

//         let seed_max_pushes = max_pushes - seed_max_pushes_offset;
//         for _ in 0..seed_max_pushes {
//             for level_idx in seed_button {
//                 seed_machine_state.levels[*level_idx as usize] += 1;
//             }
//             seed_machine_state.num_pushes += 1;
//         }

//         println!("seeded {:?}", seed_machine_state);

//         let mut states = VecDeque::new();
//         states.push_back(seed_machine_state);

//         while !states.is_empty() {
//             let state = states.pop_front().unwrap();
//             for button in &machine.buttons {
//                 let mut new_state = state.clone();
//                 for level_idx in button {
//                     new_state.levels[*level_idx as usize] += 1;
//                 }
//                 new_state.num_pushes += 1;
//                 if new_state.levels.eq(&machine.joltage_levels) {
//                     println!("{}", new_state.num_pushes);
//                     return Some(new_state.num_pushes);
//                 }
//                 let mut any_over = false;
//                 for i in 0..new_state.levels.len() {
//                     if new_state.levels[i] > machine.joltage_levels[i] {
//                         any_over = true;
//                         break;
//                     }
//                 }
//                 if !any_over {
//                     states.push_back(new_state);
//                 }
//             }
//         }

//         None
//     };

//     let get_fewest_pushes_1 = |machine: &Machine| {
//         let mut max_pushes_offset = 0;
//         loop {
//             let mut best_fewest_pushes = None;
//             for (button_idx, _) in machine.buttons.iter().enumerate() {
//                 if let Some(fewest_pushes) = get_fewest_pushes_0(machine, button_idx, max_pushes_offset) {
//                     if best_fewest_pushes.is_none() || fewest_pushes < best_fewest_pushes.unwrap() {
//                         best_fewest_pushes = Some(fewest_pushes);
//                     }
//                 }
//             }
//             if best_fewest_pushes.is_some() {
//                 return best_fewest_pushes.unwrap();
//             }
//             max_pushes_offset += 1;
//         }
//     };
     
//     input.machines.iter().map(|m| get_fewest_pushes_1(m)).sum()
// }
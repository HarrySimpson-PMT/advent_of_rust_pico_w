use crate::solver::Solver;
use core::fmt::Write;
use heapless::{String, Vec, FnvIndexMap as HashMap, Deque as VecDeque};

pub struct Day;

struct Machine {
    id: u8,
    ind_lights: Vec<bool, 10>,
    buttons: Vec<Vec<u8, 10>, 10>,
    joltage: Vec<u16, 10>,
}

fn build_machines(input: &Vec<&str, 200>) -> Vec<Machine, 200> {
    let mut machines = Vec::new();

    for (i, line) in input.iter().enumerate() {
        let parts: Vec<&str, 15> = line.split_whitespace().collect();

        let light_str = parts[0];
        let ind_lights: Vec<bool, 10> = light_str[1..light_str.len() - 1]
            .chars()
            .map(|c| c == '#')
            .collect();

        let joltage_idx = parts
            .iter()
            .rposition(|&s| s.contains('{'))
            .expect("joltage set not found");

        let mut buttons = Vec::new();
        for &btn in &parts[1..joltage_idx] {
            if btn.starts_with('(') && btn.ends_with(')') {
                let inner = &btn[1..btn.len() - 1];
                let row: Vec<u8, 10> = inner
                    .split(',')
                    .filter_map(|s| s.parse::<u8>().ok())
                    .collect();
                let try_input = buttons.push(row);
                if try_input.is_err() {
                    defmt::error!("Button vector full, cannot add more buttons");
                    break;
                }
            }
        }

        let joltage_str = parts.last().unwrap();
        let joltage_inner = &joltage_str[1..joltage_str.len() - 1]; // strip { and }
        let joltage: Vec<u16, 10> = joltage_inner
            .split(',')
            .filter_map(|s| s.parse::<u16>().ok())
            .collect();

        let try_input = machines.push(Machine {
            id: i as u8,
            ind_lights,
            buttons,
            joltage,
        });
        if try_input.is_err() {
            defmt::error!("Machine vector full, cannot add more machines");
            break;
        }
    }

    machines
}

impl Solver for Day {
    fn solve(input: String<30000>) -> String<100> {
        defmt::info!("Solving Day x...");
        let mut output = String::<100>::new();
        let mut result_a: u64 = 0;
        let mut result_b: u64 = 0;
        let lines: Vec<&str, 200> = input.split('\n').collect();
        let machines: Vec<Machine, 200> = build_machines(&lines);
        fn press_button(state: &mut Vec<bool, 10>, button: &Vec<u8, 10>, _joltage: &Vec<u16, 10>) {
            for &pos in button {
                state[pos as usize] = !state[pos as usize];
            }
        }
        for i in 0..machines.len() {
            let m = &machines[i];
            // print_machine(m);
            let mut base_state: Vec<bool,10> = Vec::new();
            for _ in 0..m.ind_lights.len() {
                base_state.push(false).unwrap();
            }
            let mut state_map: HashMap<Vec<bool,10>, u32, 1024> = HashMap::new();
            let mut queue: VecDeque<(Vec<bool,10>, u32),1000> = VecDeque::new();
            queue.push_back((base_state.clone(), 0));
            while let Some((state, presses)) = queue.pop_front() {
                if let Some(&existing_presses) = state_map.get(&state) {
                    if presses >= existing_presses {
                        continue;
                    }
                }
                state_map.insert(state.clone(), presses);
                if state == m.ind_lights {
                    result_a += presses as u64;
                    break;
                }
                for button in &m.buttons {
                    let mut new_state = state.clone();
                    press_button(&mut new_state, button, &m.joltage);
                    queue.push_back((new_state, presses + 1));
                }
            }
        }
        if output.push_str("Part A: ").is_ok() {
            if write!(output, "{} ", result_a).is_ok() {}
        } else {
            output.clear();
            output.push_str("Part A: Error ").ok();
        }
        if output.push_str("Part B: ").is_ok() {
            if write!(output, "{}", result_b).is_ok() {}
        } else {
            output.clear();
            output.push_str("Part B: Error ").ok();
        }

        output
    }
}

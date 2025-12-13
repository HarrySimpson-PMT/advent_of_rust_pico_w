use crate::solver::Solver;
use core::{fmt::Write};
use heapless::{FnvIndexMap, FnvIndexSet, String, Vec};

pub struct Day;

impl Solver for Day {
    fn solve(input: String<30000>) -> String<100> {
        let mut output = String::<100>::new();
        let mut result_b: u64 = 0;
        let lines: Vec<&str, 150> = input.split('\n').collect();

        // let mut map: HashMap<(usize, usize), u64> = HashMap::new();
        let mut map: FnvIndexMap<(usize, usize), u64, 2048> = FnvIndexMap::new();
        let mut visited: FnvIndexSet<(usize, usize), 2048> = FnvIndexSet::new();
        let mut start_x = 0;
        let start_y = lines[0].find('S').unwrap();
        loop {
            let c = lines[start_x].as_bytes()[start_y] as char;
            if c == '^' {
                let _ = visited.insert((start_x, start_y));
                let _ = map.insert((start_x, start_y), 1);
                break;
            }
            start_x += 1;
        }
        for x in start_x..lines.len() {
            for y in 0..lines[0].len() {
                if !map.contains_key(&(x, y)) {
                    continue;
                }
                let currnt_value = *map.get(&(x, y)).unwrap();
                if y > 0 {
                    let mut nx = x + 1;
                    loop {
                        let nc = lines[nx].as_bytes()[y - 1] as char;
                        if nc == '^' {
                            let _ = visited.insert((nx, y - 1));
                            if map.contains_key(&(nx, y - 1)) {
                                let entry = map.get(&(nx, y - 1));
                                let new_value = entry.unwrap() + currnt_value;
                                let _ = map.insert((nx, y - 1), new_value);
                            } else {
                                let _ = map.insert((nx, y - 1), currnt_value);
                            }
                            break;
                        } else if nx + 1 >= lines.len() {
                            result_b += currnt_value as u64;
                            break;
                        }
                        nx += 1;
                    }
                }
                if y + 1 < lines[0].len() {
                    let mut nx = x + 1;
                    loop {
                        let nc = lines[nx].as_bytes()[y + 1] as char;
                        if nc == '^' {
                            let _ = visited.insert((nx, y + 1));
                            if map.contains_key(&(nx, y + 1)) {
                                let entry = map.get(&(nx, y + 1));
                                let new_value = entry.unwrap() + currnt_value;
                                let _ = map.insert((nx, y + 1), new_value);
                            } else {
                                let _ = map.insert((nx, y + 1), currnt_value);
                            }
                            break;
                        } else if nx + 1 >= lines.len() {
                            result_b += currnt_value as u64;
                            break;
                        }
                        nx += 1;
                    }
                }
            }
        }
        let result_a = visited.len() as u64;

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

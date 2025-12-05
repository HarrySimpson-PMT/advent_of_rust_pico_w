use crate::solver::Solver;
use defmt::info;
use heapless::{String, Vec};

pub struct Day01;

impl Solver for Day01 {
    fn solve(input: &mut String<30000>) -> String<100> {
        info!("Solving Day01 with input length");
        let mut output = String::<100>::new();
        if input.is_empty() {
            output.push_str("Error: Empty input").ok();
            return output;
        }
        let mut position: i32 = 50;
        let mut last_position: i32 = 50;
        let mut result_a: i32 = 0;
        let mut result_b: i32 = 0;
        let lines: Vec<&str, 5000> = input.split('\n').collect();
        info!("Lines: {}", lines.len());
        for line in lines {
            let (turn, dist) = line.split_at(1);
            let dist: i32 = dist.parse().unwrap();
            match turn {
                "L" => {
                    position -= dist;
                    let last_hundreds = last_position / 100;
                    let current_hundreds = position / 100;
                    if last_hundreds == current_hundreds && position < 0 && last_position != 0 {
                        result_b += 1;
                    }
                    if last_hundreds != current_hundreds {
                        let extra = if last_position != 0 { 1 } else { 0 };
                        let diff = (current_hundreds - last_hundreds).abs() + extra;
                        result_b += diff;
                    }
                    if position == 0 {
                        result_b += 1;
                    }
                }
                "R" => {
                    position += dist;
                    let last_hundreds = last_position / 100;
                    let current_hundreds = position / 100;
                    if last_hundreds != current_hundreds {
                        let diff = (current_hundreds - last_hundreds).abs();
                        result_b += diff;
                    }
                }
                _ => {
                    defmt::warn!("Unknown turn direction: {}", turn);
                }
            }
            if position % 100 == 0 {
                result_a += 1;
            }
            position = position % 100;
            if position < 0 {
                position += 100;
            }
            last_position = position;
        }

        use core::fmt::Write;
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

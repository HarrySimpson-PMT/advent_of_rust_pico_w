use core::result;

use crate::solver::Solver;
use defmt::info;
use heapless::{String, Vec};
use heapless::binary_heap::{BinaryHeap, Min};
use heapless::FnvIndexMap;

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
        let mut result_a: u64 = 0;
        let mut result_b: u64 = 0;
        let lines: Vec<&str, 30000> = input.split('\n').collect(); 
        info!("Lines: {}", lines.len());
        for line in lines{
            let (turn, dist) = line.split_at(1);
            let dist: i32 = dist.parse().unwrap();
            match turn {
                "L" => {
                    position -= dist;
                }
                "R" => {
                    position += dist;
                }
                _ => {
                    defmt::warn!("Unknown turn direction: {}", turn);
                }
            }
            if position % 100 == 0{
                result_a += 1;
            }
        }

        
        
        
        
        use core::fmt::Write;
        if output.push_str("Part A: ").is_ok() {
            if write!(output, "{} ", result_a).is_ok() {
            }
        }        
        else {
            output.clear();
            output.push_str("Part A: Error ").ok();
        }
        if output.push_str("Part B: ").is_ok() {
            if write!(output, "{}", result_b).is_ok() {
            }
        }        
        else {
            output.clear();
            output.push_str("Part B: Error ").ok();
        }

        output
    }
}


use super::Solver;
use core::fmt::Write;
#[allow(unused_imports)]
use heapless::{String, Vec, FnvIndexMap};
#[allow(unused_imports)]
use heapless::binary_heap::{BinaryHeap, Min};
pub struct Day05;

impl Solver for Day05 {
    fn solve(input: &String<20000>) -> String<100> {
        let mut output = String::<100>::new();
        let mut first_result: String<50> = String::new();
        let mut second_result: String<50> = String::new();
        if input.is_empty() {
            output.push_str("Error: Empty input").ok();
            return output;
        }


        

        first_result.push_str("Not implemented").ok();
        second_result.push_str("Not implemented").ok();
        if output.push_str("Part A: ").is_ok() {
            if write!(output, "{} ", first_result).is_ok() {
            }
        }        
        else {
            output.clear();
            output.push_str("Part A: Error ").ok();
        }
        if output.push_str("Part B: ").is_ok() {
            if write!(output, "{}", second_result).is_ok() {
            }
        }        
        else {
            output.clear();
            output.push_str("Part B: Error ").ok();
        }

        output
    }
}


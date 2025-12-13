use crate::solver::Solver;
use heapless::{String, Vec};
use core::fmt::Write;

pub struct Day;

#[derive(Debug, Clone, PartialEq, Eq)] // Add traits as needed for your use case
struct Range {
    start: u64,
    end: u64,
}

impl Solver for Day {
    fn solve(input: String<30000>) -> String<100> {
        defmt::info!("Solving Day x...");
        let mut output = String::<100>::new();
        let mut result_a: u64 = 0;
        let mut result_b: u64 = 0;
        

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

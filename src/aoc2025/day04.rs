use crate::solver::SolverOwned;
use crate::solver::Solver;
use defmt::info;
use embassy_rp::gpio::Input;
use heapless::{String, Vec};

pub struct Day;

impl SolverOwned for Day {
    fn solve(input: String<30000>) -> (String<30000>, String<100>) {
        let mut output = String::<100>::new();
        let result_a: u64 = 0;
        let result_b: u64 = 0;
        let line_len = input.find('\n').unwrap();
        if let Some(nl_pos) = input.as_str().bytes().position(|b| b == b'\n') {
            let target = nl_pos + 2;
            if target < input.len() {
                // This is the only safe in-place mutation API in heapless 0.8
                input.as_mut_vec()[target] = b'Q';
            }
        }
        //print lines
        let lines: Vec<&str, 5000> = input.split('\n').collect();
        for line in lines {
            info!("Line: {}", line);
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

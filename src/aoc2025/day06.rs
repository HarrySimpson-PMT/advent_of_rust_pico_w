use crate::solver::Solver;
use heapless::{String, Vec};

pub struct Day;

#[derive(Debug, Clone, PartialEq, Eq)] // Add traits as needed for your use case
struct Range {
    start: u64,
    end: u64,
}

impl Solver for Day {
    fn solve(input: String<30000>) -> String<100> {
        let mut output = String::<100>::new();
        let mut result_a: u64 = 0;
        let mut result_b: u64 = 0;
        let mut eq_1: Vec<String<10>, 5> = Vec::new();
        let mut eq_2: Vec<String<10>, 5> = Vec::new();
        for _ in 0..5 {
            eq_1.push(String::<10>::new()).ok();
            eq_2.push(String::<10>::new()).ok();
        }
        let line_len = input.find('\n').unwrap();
        let mut col_x = 0;
        for column in 0..line_len {
            let mut all_blank: bool = true;
            for line in 0..5 {
                let idx = line * (line_len + 1) + column;
                if idx < input.len() {
                    let c = input.chars().nth(idx).unwrap();
                    eq_2[col_x].push(c).ok();
                    if c != ' ' {
                        all_blank = false;
                    }
                    eq_1[line].push(c).ok();
                }
            }
            col_x += 1;
            if all_blank || column == line_len - 1 {
                col_x = 0;
                let lines = 5;
                let operator = eq_1[lines-1].chars().nth(0).unwrap_or(' ');
                eq_2[0].pop();
                let mut result_eq_a: u64 = 0;
                let mut result_eq_b: u64 = 0;
                if operator == '*' {
                    result_eq_a = 1;
                    result_eq_b = 1;
                }
                for line_number in 0..lines-1 {
                    let num_1: u64 = eq_1[line_number].trim().parse().unwrap_or(0);
                    let num_2: u64 = eq_2[line_number].trim().parse().unwrap_or(0);
                    if operator == '+' {
                        result_eq_a += num_1;
                        result_eq_b += num_2;
                    } else if operator == '*' {
                        result_eq_a *= num_1;
                        if num_2 == 0 {
                            continue;
                        }
                        result_eq_b *= num_2;
                    }
                }
                result_a += result_eq_a;
                result_b += result_eq_b;
                for i in 0..5 {
                    eq_1[i].clear();
                    eq_2[i].clear();
                }
            }
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

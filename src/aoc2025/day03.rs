use crate::solver::Solver;
use heapless::{String, Vec};

pub struct Day;

impl Solver for Day {
    fn solve(input: &mut String<30000>) -> String<100> {
        let mut output = String::<100>::new();
        let mut result_a: i64 = 0;
        let mut result_b: i64 = 0;
        let mut stack: Vec<u32, 3> = Vec::new();
        let mut stack2: Vec<u32, 13> = Vec::new();
        for line in input.split('\n') {
            for i in 0..line.len() - 1 {
                if let Some(digit) = line.chars().nth(i).and_then(|c| c.to_digit(10)) {
                    while !stack.is_empty() && *stack.last().unwrap() < digit {
                        stack.pop();
                    }
                    if stack.len() >= 2 {
                        continue;
                    } else {
                        let _ = stack.push(digit);
                    }
                }
            }
            for i in 0..line.len() {
                if let Some(digit) = line.chars().nth(i).and_then(|c| c.to_digit(10)) {
                    while !stack2.is_empty()
                        && *stack2.last().unwrap() < digit
                        && stack2.len() + line.len() - i > 12
                    {
                        stack2.pop();
                    }

                    if stack2.len() >= 12 {
                        continue;
                    } else {
                        let _ = stack2.push(digit);
                    }
                }
            }
            let last_char = line.chars().last().and_then(|c| c.to_digit(10));
            if stack.len() < 2 {
                let _ = stack.push(last_char.unwrap_or(0));
            } else if *stack.last().unwrap_or(&0) < last_char.unwrap_or(0) {
                stack.pop();
                let _ = stack.push(last_char.unwrap_or(0));
            }
            let current_num: i64 = stack.iter().fold(0, |acc, &d| acc * 10 + d as i64);
            result_a += current_num;
            stack.clear();

            let mut current: String<12> = String::new();
            while !stack2.is_empty() {
                let d = stack2.pop().unwrap();
                let _ = write!(current, "{}", d);
            }
            let current: String<12> = current.chars().rev().collect();
            result_b += current.parse::<i64>().unwrap();
            stack2.clear();
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

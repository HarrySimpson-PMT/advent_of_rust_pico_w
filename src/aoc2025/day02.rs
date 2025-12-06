use crate::solver::Solver;
use defmt::info;
use heapless::{String, Vec};

pub struct Day;

impl Solver for Day {
    fn solve(input: &mut String<30000>) -> String<100> {
        info!("Solving Day01 with input length");
        let mut output = String::<100>::new();
        let mut result_a: u64 = 0;
        let mut result_b: u64 = 0;
        //lines are comming in like
        let range_strings: Vec<&str, 5000> = input.split(',').collect();
        info!("Ranges: {}", range_strings.len());
        for range_string in range_strings {
            let (start, end) = range_string.split_at(range_string.find('-').unwrap());
            let start: u64 = start.parse().unwrap();
            let end: u64 = end[1..].parse().unwrap();
            for i in start..=end {
                let mut num_str: String<20> = String::new();
                write!(num_str, "{}", i).ok();
                let chars: Vec<char, 20> = num_str.chars().collect();
                if chars[..chars.len() / 2] == chars[chars.len() / 2..] {
                    result_a += i;
                }
                for len in 1..=chars.len() / 2 {
                    let mut found_repeat = true;
                    let pattern = &chars[..len];
                    let mut index = len;
                    while index + len <= chars.len() {
                        if &chars[index..index + len] != pattern {
                            found_repeat = false;
                            break;
                        }
                        index += len;
                    }
                    if found_repeat && index == chars.len() {
                        result_b += i;
                        break;
                    }
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

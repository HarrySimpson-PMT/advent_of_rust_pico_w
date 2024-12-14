use super::Solver;
use core::fmt::Write;
#[allow(unused_imports)]
use heapless::binary_heap::{BinaryHeap, Min};
#[allow(unused_imports)]
use heapless::{FnvIndexMap, String, Vec};

pub struct Day03;

impl Solver for Day03 {
    fn solve(input: &String<20000>) -> String<100> {
        let mut output = String::<100>::new();
        let mut result_a: u64 = 0;
        let mut result_b: u64 = 0;
        let mut on = true;

        // Process the content for both parts
        let mut start = 0;
        while let Some(pos) = input[start..].find(|c: char| c == 'm' || c == 'd') {
            let offset = start + pos;

            if input[offset..].starts_with("do()") {
                on = true;
                start = offset + 4; // Move past "do()"
                continue;
            }
            if input[offset..].starts_with("don't()") {
                on = false;
                start = offset + 7; // Move past "don't()"
                continue;
            }

            if input[offset..].starts_with("mul(") {
                if let Some(end) = input[offset..].find(')') {
                    let slice = &input[offset..offset + end + 1];
                    if let Some((a, b)) = parse_mul(slice) {
                        result_a += a as u64 * b as u64;
                        if on {
                            result_b += a as u64 * b as u64;
                        }
                        start = offset + end;
                    }else {
                        start = offset + 4; // Move past "mul("
                    }

                    continue;
                } else {
                    break; // No closing parenthesis found
                }
            }
            start += 1;
        }

        // Format the output
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
fn parse_mul(input: &str) -> Option<(i32, i32)> {
    if !input.starts_with("mul(") || !input.ends_with(")") {
        return None;
    }

    // Extract the content inside the parentheses
    let content = &input[4..input.len() - 1]; // Remove "mul(" and ")"
    
    // Ensure the content contains exactly one comma
    if content.matches(',').count() != 1 {
        return None;
    }

    let mut parts = content.split(',');

    // Extract the two parts
    if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
        // Check if each part is numeric and at most 6 digits
        if first.len() > 3 || second.len() > 3 || !first.chars().all(|c| c.is_digit(10)) || !second.chars().all(|c| c.is_digit(10)) {
            return None;
        }

        // Parse the numbers
        if let (Ok(a), Ok(b)) = (str_to_i32(first), str_to_i32(second)) {
            return Some((a, b));
        }
    }

    None
}


// Custom integer parser
fn str_to_i32(s: &str) -> Result<i32, ()> {
    let mut result = 0;
    let mut negative = false;

    for (i, c) in s.chars().enumerate() {
        if i == 0 && c == '-' {
            negative = true;
            continue;
        }
        if let Some(digit) = c.to_digit(10) {
            result = result * 10 + digit as i32;
        } else {
            return Err(()); // Invalid character
        }
    }

    if negative {
        result = -result;
    }

    Ok(result)
}

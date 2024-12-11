use super::Solver;
use defmt::println;
use heapless::{String, Vec}; // U128 for the Vec capacity

pub struct Day01;

impl Solver for Day01 {
    fn solve(input: &String<20000>) -> String<100> {
        let mut output = String::<100>::new();

        if input.is_empty() {
            output.push_str("Error: Empty input").ok();
            return output;
        }

        // Priority queues using heapless Vec
        let mut pq1: Vec<i32, 1000> = Vec::new(); // Priority queue for first numbers
        let mut pq2: Vec<i32, 1000> = Vec::new(); // Priority queue for second numbers

        // Split input into lines
        let lines: Vec<&str, 1000> = input.split('\n').collect();
        defmt::info!("Lines: {}", lines.len());
        for line in lines {
            let parts: Vec<&str, 5> = line.split_whitespace().collect();
            if let (Some(&num1_str), Some(&num2_str)) = (parts.get(0), parts.get(1)) {
                if let (Ok(num1), Ok(num2)) = (num1_str.parse::<i32>(), num2_str.parse::<i32>()) {
                    pq1.push(num1).ok(); // Push into priority queue
                    pq2.push(num2).ok();
                } else {
                    defmt::warn!("Error parsing numbers in line: {}", line);
                }
            } else {
                defmt::warn!("Error splitting line: {}", line);
            }
        }

        // Calculate the total sum of absolute differences
        let total_diff = calculate_sum_of_abs_differences(&pq1, &pq2);
        use core::fmt::Write;
        // Write the result into the output
        if output.push_str("Total: ").is_ok() {
            if write!(output, "{}", total_diff).is_ok() {
                return output;
            }
        }
        else {
            output.clear();
            output.push_str("Error: Output overflow").ok();
        }

        output
    }
}

/// Helper function to calculate sum of absolute differences
fn calculate_sum_of_abs_differences(pq1: &Vec<i32, 1000>, pq2: &Vec<i32, 1000>) -> i64 {
    let mut total_diff: i64 = 0;

    for (&num1, &num2) in pq1.iter().zip(pq2.iter()) {
        total_diff += (num1 - num2).abs() as i64;
    }

    total_diff
}

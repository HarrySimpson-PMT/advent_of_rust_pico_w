use super::Solver;
use heapless::{String, Vec};
use heapless::binary_heap::{BinaryHeap, Min};
use heapless::FnvIndexMap;

pub struct Day01;

impl Solver for Day01 {
    fn solve(input: &mut String<30000>) -> String<100> {
        let mut output = String::<100>::new();

        if input.is_empty() {
            output.push_str("Error: Empty input").ok();
            return output;
        }

        let mut pq1: BinaryHeap<i32, Min, 1000> = BinaryHeap::new(); 
        let mut pq2: BinaryHeap<i32,Min, 1000> = BinaryHeap::new();
        let mut first_numbers: Vec<i32, 1000> = Vec::new();
        let mut second_number_counts: FnvIndexMap<i32, i32, 1024> = FnvIndexMap::new();

        let lines: Vec<&str, 1000> = input.split('\n').collect();
        defmt::info!("Lines: {}", lines.len());
        for line in lines {
            let parts: Vec<&str, 5> = line.split_whitespace().collect();
            if let (Some(&num1_str), Some(&num2_str)) = (parts.get(0), parts.get(1)) {
                if let (Ok(num1), Ok(num2)) = (num1_str.parse::<i32>(), num2_str.parse::<i32>()) {
                    pq1.push(num1).ok();
                    pq2.push(num2).ok();
                    first_numbers.push(num1).ok();
                    if second_number_counts.contains_key(&num2) {
                        *second_number_counts.get_mut(&num2).unwrap() += 1;
                    } else {
                        let _ = second_number_counts.insert(num2, 1);
                    }
                } else {
                    defmt::warn!("Error parsing numbers in line: {}", line);
                }
            } else {
                defmt::warn!("Error splitting line: {}", line);
            }
        }
        let total_diff = calculate_sum_of_abs_differences(&mut pq1, &mut pq2);
        use core::fmt::Write;



        let mut total: u64 = 0;

        // Calculate the total
        for &item in &first_numbers {
            if let Some(&count) = second_number_counts.get(&item) {
                total += (item as u64) * (count as u64);
            }
        }


        if output.push_str("Part A: ").is_ok() {
            if write!(output, "{} ", total_diff).is_ok() {
            }
        }        
        else {
            output.clear();
            output.push_str("Part A: Error ").ok();
        }
        if output.push_str("Part B: ").is_ok() {
            if write!(output, "{}", total).is_ok() {
            }
        }        
        else {
            output.clear();
            output.push_str("Part B: Error ").ok();
        }

        output
    }
}

fn calculate_sum_of_abs_differences(pq1: &mut BinaryHeap<i32,Min, 1000>, pq2: &mut BinaryHeap<i32,Min, 1000>) -> u64 {
    let mut total_diff: u64 = 0;
    while let (Some(num1), Some(num2)) = (pq1.pop(), pq2.pop()) {
        let cur_diff = (num1 - num2).abs() as u64;
        defmt::info!("Diff: {}", cur_diff);
        total_diff += cur_diff;        
    }
    total_diff
}

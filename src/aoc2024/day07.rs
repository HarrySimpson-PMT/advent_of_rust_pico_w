use crate::solver::Solver;
use core::fmt::Write;
#[allow(unused_imports)]
use heapless::{String, Vec, FnvIndexMap};
#[allow(unused_imports)]
use heapless::binary_heap::{BinaryHeap, Min};
pub struct Day;


impl Solver for Day {
    fn solve(input: String<30000>) -> String<100> {
        let mut output = String::<100>::new();
        let mut first_result: u64 = 0;
        let mut second_result: u64 = 0;
        if input.is_empty() {
            output.push_str("Error: Empty input").ok();
            return output;
        }
        for line in input.lines() {
            let mut part = line.split(":");
            let expected_result: u64 = part.next().unwrap().parse().unwrap();
            let deltas: Vec<u64, 12> = part.next().unwrap().trim().split(" ").map(|x| x.parse().unwrap()).collect();
            if evaluate_combinations(&deltas, 0, 0, expected_result) {
                first_result += expected_result;
            }
            if evaluate_combinations2(&deltas, 1, deltas[0], expected_result as u64) {
                second_result += expected_result;
            }
        }

        fn evaluate_combinations(
            numbers: &Vec<u64, 12>,
            index: u8,
            current_value: u64,
            target: u64) -> bool {
            if index == numbers.len() as u8 {
                return current_value == target;
            }
            if current_value>target {
                return false;
            }
            let next_value = numbers[index as usize];
            evaluate_combinations(numbers, index + 1, current_value + next_value, target) ||
            evaluate_combinations(numbers, index + 1, current_value * next_value, target)
        }

        fn digit_count(mut n: u64) -> u64 {
            if n == 0 {
                return 1;
            }
            let mut count = 0;
            while n > 0 {
                n /= 10;
                count += 1;
            }
            count
        }
        fn evaluate_combinations2(
            numbers: &Vec<u64, 12>,
            mut index: usize,
            acc: u64,
            target: u64,
        ) -> bool {
            if index == numbers.len() {
                return acc == target
            }
            let current_number = numbers[index];
            if acc>target {
                return false
            }
            index += 1;
    
            if evaluate_combinations2(numbers, index , acc + current_number, target) {
                return true;
            }
    
            if evaluate_combinations2(numbers, index , acc * current_number, target) {
                return true;
            }
    
            let merged = concat(acc, current_number);
            if evaluate_combinations2(numbers, index, merged, target) {
                return true;
            }
    
            false
        }
    
        fn concat(a: u64, b: u64) -> u64 {
            let digits = digit_count(b);
            a * 10_u64.pow(digits as u32) + b
        }

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


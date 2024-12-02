#![no_std]
#![no_main]
#![allow(async_fn_in_trait)]

use defmt::*;
use heapless::{Vec, String};

const MAX_CHARS: usize = 8;
const MAX_LINES: usize = 2000;
const MAX_RESULT_LEN: usize = 256;

pub fn solve(input_lines: &Vec<[u8; MAX_CHARS], MAX_LINES>, line_count: usize) -> String<MAX_RESULT_LEN> {
    use core::fmt::Write;

    info!("Starting...");

    let mut result: String<MAX_RESULT_LEN> = String::new();
    let mut count = 1;

    info!("Lines to process: {}", line_count);

    for i in 1..line_count {
        let prev_raw = core::str::from_utf8(&input_lines[i - 1]).unwrap_or("");
        let curr_raw = core::str::from_utf8(&input_lines[i]).unwrap_or("");
    
        // Trim null bytes by stopping at the first null byte
        let prev = prev_raw.trim_end_matches('\0');
        let curr = curr_raw.trim_end_matches('\0');
    
        info!("Line {}: prev = {:?}, curr = {:?}", i, prev, curr);
    
        match (prev.parse::<i32>(), curr.parse::<i32>()) {
            (Ok(prev_val), Ok(curr_val)) => {
                if curr_val > prev_val {
                    count += 1;
                    info!("Increasing: {} > {}", curr_val, prev_val);
                } else {
                    info!("Not increasing: {} <= {}", curr_val, prev_val);
                }
            }
            _ => {
                info!("Failed to parse prev or curr as integer");
            }
        }
    }
    
    

    writeln!(result, "Count of increasing values: {}", count).unwrap();
    result
}
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
        let lines: Vec<&str, 5000> = input.split('\n').collect();
        let mut result_a: i64 = 0;
        let mut result_b: u64 = 0;

        let mut split_index = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                split_index = i;
                break;
            }
        }
        let (range_strings, id_strings) = lines.split_at(split_index);
        let mut ids: Vec<u64, 1000> = Vec::new();
        for id_str in id_strings {
            if id_str.trim().is_empty() {
                continue;
            }
            let id = id_str.trim().parse().unwrap();
            let _  = ids.push(id);
        }
        for id in ids {
            for range_str in range_strings {
                let parts: Vec<&str,2> = range_str.split('-').collect();
                if parts.len() != 2 {
                    continue;
                }
                let start: u64 = parts[0].trim().parse().unwrap();
                let end: u64 = parts[1].trim().parse().unwrap();
                if id >= start && id <= end {
                    result_a += 1;
                    break;
                }
            }
        }
        let mut ranges: Vec<Range, 1000> = Vec::new();
        for range_str in range_strings {
            if range_str.trim().is_empty() {
                continue;
            }
            let parts: Vec<&str, 40> = range_str.split('-').collect();
            if parts.len() != 2 {
                continue;
            }
            let start: u64 = parts[0].trim().parse().unwrap();
            let end: u64 = parts[1].trim().parse().unwrap();
            let _ = ranges.push(Range { start, end });
        }
        ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));

        let mut collapsed_ranges: Vec<Range, 100> = Vec::new();
        for range in ranges {
            if collapsed_ranges.is_empty() {
                let _ = collapsed_ranges.push(range);
            } else {
                let last_range = collapsed_ranges.last_mut().unwrap();
                if range.start <= last_range.end + 1 {
                    if range.end > last_range.end {
                        last_range.end = range.end;
                    }
                } else {
                    let _ = collapsed_ranges.push(range);
                }
            }
        }
        for range in collapsed_ranges {
            result_b += range.end - range.start + 1;
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

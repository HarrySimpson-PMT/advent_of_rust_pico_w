use super::Solver;
use core::{fmt::Write, num};
#[allow(unused_imports)]
use heapless::binary_heap::{BinaryHeap, Min};
use heapless::FnvIndexSet;
#[allow(unused_imports)]
use heapless::{FnvIndexMap, String, Vec};
pub struct Day08;

impl Solver for Day08 {
    fn solve(input: &mut String<30000>) -> String<100> {
        let mut output = String::<100>::new();
        let mut first_result: String<50> = String::new();
        let mut second_result: String<50> = String::new();
        if input.is_empty() {
            output.push_str("Error: Empty input").ok();
            return output;
        }
        let mut grid = [[0u8; 50]; 50];
        let mut result_grid = [[false; 50]; 50];

        let mut dictionary = FnvIndexMap::<u8, Vec<(u8, u8), 5>, 64>::new();
        fn char_to_value(c: char) -> Option<u8> {
            if c.is_ascii_digit() {
                Some(c as u8 - b'0')
            } else if c.is_ascii_uppercase() {
                Some(c as u8 - b'A' + 10)
            } else if c.is_ascii_lowercase() {
                Some(c as u8 - b'a' + 36)
            } else {
                None
            }
        }
        fn value_to_char(v: u8) -> Option<char> {
            match v {
                0..=9 => Some((b'0' + v) as char),      
                10..=35 => Some((b'A' + (v - 10)) as char),  
                36..=61 => Some((b'a' + (v - 36)) as char),
                _ => None,
            }
        }
        let mut y: usize = 0;
        let mut x: usize = 0;
        let mut start = 0;
        for (i, c) in input.char_indices() {
            if c == '\n' {
                let line = &input[start..i];
                for (j, c) in line.chars().enumerate() {
                    if c.is_alphanumeric() {
                        //set numeric to 0-9 and aplha to 10-35
                        let num = char_to_value(c).unwrap();

                        if let Some(values) = dictionary.get_mut(&num) {
                            if values.len() < 5 {
                                values
                                    .push((x.try_into().unwrap(), y.try_into().unwrap()))
                                    .ok();
                            } else {
                                defmt::error!("Dictionary value too long");
                            }
                        } else {
                            let mut new_vec = Vec::<(u8, u8), 5>::new();
                            new_vec
                                .push((x.try_into().unwrap(), y.try_into().unwrap()))
                                .ok();
                            dictionary.insert(num, new_vec).ok();
                            grid[y][x] = num;
                            x += 1;
                        }

                    } else {
                        defmt::warn!("Non-numeric character encountered: {}", c);
                    }
                    x += 1;
                }
                y += 1;
                x = 0;
                start = i + 1;
            }
        }

        //print the dictionary
        for (key, values) in dictionary.iter() {
            defmt::info!("Key: {}", key);
            for (x, y) in values.iter() {
                defmt::info!("x: {}, y: {}", x, y);
            }
        }

        first_result.push_str("Not implemented").ok();
        second_result.push_str("Not implemented").ok();
        if output.push_str("Part A: ").is_ok() {
            if write!(output, "{} ", first_result).is_ok() {}
        } else {
            output.clear();
            output.push_str("Part A: Error ").ok();
        }
        if output.push_str("Part B: ").is_ok() {
            if write!(output, "{}", second_result).is_ok() {}
        } else {
            output.clear();
            output.push_str("Part B: Error ").ok();
        }

        output
    }
}

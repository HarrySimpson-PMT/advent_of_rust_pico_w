use super::Solver;
use core::fmt::Write;
use heapless::{String, Vec};

pub struct Day02;

impl Solver for Day02 {
    fn solve(input: &mut String<20000>) -> String<100> {
        let mut output = String::<100>::new();

        if input.is_empty() {
            output.push_str("Error: Empty input").ok();
            return output;
        }

        let data: Vec<Vec<i8, 10>, 1000> = input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|num| num.parse::<i8>().ok())
                    .collect::<Vec<i8, 10>>() 
            })
            .filter(|vec| !vec.is_empty()) 
            .collect();

        let mut good_lines = 0;
        let mut almost_good_lines = 0;

        for line in data.iter() {
            let increasing = line[0] < line[1];
            let mut good = true;

            for i in 0..line.len()-1 {
                if increasing && line[i] >= line[i + 1]  {
                    good = false;
                    break;
                }
                if !increasing && line[i] <= line[i + 1]  {
                    good = false;
                    break;
                }
                if (line[i] - line[i + 1]).abs() > 3 {
                    good = false;
                    break;
                }
            }

            if good {
                good_lines += 1;
                continue;
            } else {
                for j in 0..line.len() {
                    let mut temp_line = line.clone();
                    temp_line.remove(j);

                    let increasing = temp_line[0] < temp_line[1];
                    good = true;

                    for i in 0..temp_line.len()-1 {
                        if increasing && temp_line[i] >= temp_line[i + 1]  {
                            good = false;
                            break;
                        }
                        if !increasing && temp_line[i] <= temp_line[i + 1]  {
                            good = false;
                            break;
                        }
                        if (temp_line[i] - temp_line[i + 1]).abs() > 3 {
                            good = false;
                            break;
                        }
                    }

                    if good {
                        almost_good_lines += 1;
                        break;
                    }
                }
            }
        }

        let first_result = good_lines;
        let second_result = good_lines + almost_good_lines;

        if output.push_str("Part A: ").is_ok() {
            write!(output, "{} ", first_result).ok();
        } else {
            output.clear();
            output.push_str("Part A: Error").ok();
        }

        if output.push_str("Part B: ").is_ok() {
            write!(output, "{}", second_result).ok();
        } else {
            output.clear();
            output.push_str("Part B: Error").ok();
        }

        output
    }
}

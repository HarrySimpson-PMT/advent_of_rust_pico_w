use crate::solver::Solver;
use core::{fmt::Write, str::FromStr};
use heapless::{String, Vec};

pub struct Day;

impl Solver for Day {
    fn solve(input: String<30000>) -> String<100> {
        let mut output = String::<100>::new();
        let mut first_result: String<50> = String::new();
        let mut second_result: String<50> = String::new();

        if input.is_empty() {
            output.push_str("Error: Empty input").ok();
            return output;
        }

        let lines: Vec<String<150>, 150> = input
            .lines()
            .map(|line| String::<150>::from_str(line).unwrap_or_else(|_| String::new()))
            .collect();

        defmt::info!("{:?}", lines);
        let target = "XMAS";
        let found_count = count_occurrences(&lines, target);
        write!(first_result, "{}", found_count).ok();

        let pattern_count = find_pattern(&lines);
        write!(second_result, "{}", pattern_count).ok();

        // Format results
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

fn count_occurrences(matrix: &Vec<String<150>, 150>, target: &str) -> usize {
    let rows = matrix.len();
    if rows == 0 {
        return 0;
    }
    let cols = matrix[0].len();
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (0, -1),  // Left
        (-1, 0),  // Up
        (1, 1),   // Down-right
        (1, -1),  // Down-left
        (-1, 1),  // Up-right
        (-1, -1), // Up-left
    ];

    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            if matrix[row].as_bytes()[col] == b'X' {
                for &(dx, dy) in &directions {
                    if check_direction(matrix, target, row as isize, col as isize, dx, dy) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn check_direction(
    matrix: &Vec<String<150>, 150>,
    target: &str,
    start_row: isize,
    start_col: isize,
    dx: isize,
    dy: isize,
) -> bool {
    let rows = matrix.len() as isize;
    let cols = matrix[0].len() as isize;
    let target_bytes = target.as_bytes();

    for (i, &ch) in target_bytes.iter().enumerate() {
        let new_row = start_row + i as isize * dx;
        let new_col = start_col + i as isize * dy;

        if new_row < 0
            || new_row >= rows
            || new_col < 0
            || new_col >= cols
            || matrix[new_row as usize].as_bytes()[new_col as usize] != ch
        {
            return false;
        }
    }

    true
}

fn find_pattern(matrix: &Vec<String<150>, 150>) -> usize {
    let rows = matrix.len();
    if rows == 0 {
        return 0;
    }
    let cols = matrix[0].len();
    let mut count = 0;

    let diagonals = [
        (-1, -1), // Top-left
        (-1, 1),  // Top-right
        (1, -1),  // Bottom-left
        (1, 1),   // Bottom-right
    ];

    for row in 0..rows {
        for col in 0..cols {
            if matrix[row].as_bytes()[col] == b'A' {
                let mut diag_chars = Vec::<u8, 4>::new();
                for &(dx, dy) in &diagonals {
                    let new_row = row as isize + dx;
                    let new_col = col as isize + dy;

                    if new_row >= 0
                        && new_row < rows as isize
                        && new_col >= 0
                        && new_col < cols as isize
                    {
                        diag_chars.push(matrix[new_row as usize].as_bytes()[new_col as usize])
                            .ok();
                    }
                }
                if diag_chars.len() == 4 {
                    let m_count = diag_chars.iter().filter(|&&c| c == b'M').count();
                    let s_count = diag_chars.iter().filter(|&&c| c == b'S').count();
                    if m_count == 2
                        && s_count == 2
                        && diag_chars[0] != diag_chars[3] 
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

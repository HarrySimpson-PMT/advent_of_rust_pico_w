use super::Solver;
use core::fmt::Write;
use heapless::{String, Vec, FnvIndexSet};

pub struct Day06;

impl Solver for Day06 {
    fn solve(input: &String<20000>) -> String<100> {
        let mut output = String::<100>::new();
        let mut first_result: String<50> = String::new();
        let mut second_result: String<50> = String::new();

        if input.is_empty() {
            output.push_str("Error: Empty input").ok();
            return output;
        }

        const GRID_SIZE: usize = 130;
        const GRID_AREA: usize = GRID_SIZE * GRID_SIZE;

        let mut grid = Vec::<char, GRID_AREA>::new();
        let mut start_pos = (0, 0);

        parse_grid(input, &mut grid, &mut start_pos, GRID_SIZE);

        let part_a_result = traverse_grid(&mut grid, start_pos, GRID_SIZE);
        
        for y in 0..GRID_SIZE {
            let row = &grid[y * GRID_SIZE..(y + 1) * GRID_SIZE];
            let mut row_str = String::<130>::new();
            for c in row {
                row_str.push(*c).ok();
            }
            defmt::info!("{}", row_str);
        }
        
        
        // let part_b_result = test_obstacles(&grid, start_pos, GRID_SIZE);

        let part_b_result = 0;

        write!(first_result, "{}", part_a_result).ok();
        write!(second_result, "{}", part_b_result).ok();

        if output.push_str("Part A: ").is_ok() {
            write!(output, "{} ", first_result).ok();
        } else {
            output.clear();
            output.push_str("Part A: Error ").ok();
        }

        if output.push_str("Part B: ").is_ok() {
            write!(output, "{}", second_result).ok();
        } else {
            output.clear();
            output.push_str("Part B: Error ").ok();
        }

        output
    }
}

fn parse_grid(input: &String<20000>, grid: &mut Vec<char, 16900>, start_pos: &mut (usize, usize), width: usize) {
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                *start_pos = (x, y);
                grid.push('X').ok();
            }else{
                grid.push(c).ok();
            }
        }
    }
}

fn traverse_grid(grid: &mut Vec<char, 16900>, start_pos: (usize, usize), width: usize) -> usize {
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut visited: FnvIndexSet<(usize, u8),  16384> = FnvIndexSet::new();
    let mut dir_index = 0;
    let (mut x, mut y) = start_pos;

    while y < width && x < width {
        let idx = y * width + x;

        visited.insert((idx, dir_index)).ok();        

        grid[idx] = 'X';

        let (dx, dy) = directions[dir_index as usize];
        let next_x = x as isize + dx;
        let next_y = y as isize + dy;

        if next_x < 0 || next_y < 0 || next_x >= width as isize || next_y >= width as isize {
            break;
        }

        let nx = next_x as usize;
        let ny = next_y as usize;
        let next_idx = ny * width + nx;

        if grid[next_idx] == '#' {
            defmt::info!("Turning right");
            dir_index = (dir_index + 1) % 4;
        } else {
            //moving to the next position
            defmt::info!("Moving to the next position {} {}", nx, ny);
            x = nx;
            y = ny;
        }
    }

    visited.len()
}


fn encode_position(x: usize, y: usize, dir: u8, width: usize) -> u32 {
    (((y * width + x) << 2) as u32) | (dir as u32)
}

fn decode_position(encoded: u32, width: usize) -> (usize, usize, u8) {
    let dir = (encoded & 0b11) as u8;
    let pos = encoded >> 2;
    let x = (pos % width as u32) as usize;
    let y = (pos / width as u32) as usize;
    (x, y, dir)
}

fn mark_visited(visited: &mut [bool], x: usize, y: usize, dir: u8, width: usize) {
    let encoded = encode_position(x, y, dir, width);
    visited[encoded as usize] = true;
}

fn is_visited(visited: &[bool], x: usize, y: usize, dir: u8, width: usize) -> bool {
    let encoded = encode_position(x, y, dir, width);
    visited[encoded as usize]
}
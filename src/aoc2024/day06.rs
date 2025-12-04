use crate::solver::Solver;
use core::fmt::Write;
use heapless::String;
use heapless::FnvIndexSet;

pub struct Day06;

impl Solver for Day06 {
    fn solve(input: &mut String<30000>) -> String<100> {
        const GRID_SIZE: usize = 130;
        const MAX_VISITS: usize = 16900 * 4;

        let mut visited = [false; MAX_VISITS];
        let mut grid = [b'.'; GRID_SIZE * GRID_SIZE];

        //track (x,y) set of up to 3000 loop making blocks
        let mut loop_blocks = FnvIndexSet::<(usize, usize), 2048>::new();

        let mut cur_pos = (0, 0);
        let mut cur_dir = 0;
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // N, E, S, W (x, y)
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '^' {
                    cur_pos = (x, y);
                    grid[y * GRID_SIZE + x] = b'X';
                } else {
                    grid[y * GRID_SIZE + x] = c as u8;
                }
            }
        }
        update_input_in_place(input, cur_pos.0, cur_pos.1, 'X', GRID_SIZE);

        let mut result_a = 0;
        let mut result_b = 0;
        
        let mut simulate = true; 
        let mut sim_start_step = 0;

        loop {
            let mut current_step = sim_start_step;
            //defmt the current step, position and direction
            defmt::info!("Step: {}, Pos: {},{} Dir: {}", current_step, cur_pos.0, cur_pos.1, cur_dir);

            if simulate {
                visited.fill(false); 
                simulate = false;
                sim_start_step += 1;
                for (y, line) in input.lines().enumerate() {
                    for (x, c) in line.chars().enumerate() {
                        grid[y * GRID_SIZE + x] = c as u8;
                    }
                }
            }else{
                break;
            }
            let mut simulate_loop_block_pos = (0, 0);

            let mut x = cur_pos.0;
            let mut y = cur_pos.1;
            let mut dir = cur_dir;

            loop {                
                mark_visited(&mut visited, x, y, dir as u8, GRID_SIZE);

                let (dx, dy) = directions[dir];
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                
                if nx < 0 || ny < 0 || nx as usize >= GRID_SIZE || ny as usize >= GRID_SIZE {
                    break;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if current_step == sim_start_step {
                    simulate = true;
                    //check if the input positiong has been visited with an X
                    if input.chars().nth(ny * (GRID_SIZE + 1) + nx).unwrap() == 'X' {
                        break;
                    }
                    grid[ny * GRID_SIZE + nx] = b'#';
                    simulate_loop_block_pos = (nx, ny);
                }

                let next_idx = ny * GRID_SIZE + nx;

                if grid[next_idx] == b'#' {
                    dir = (dir + 1) % 4;
                } else {
                    if simulate {
                        if is_visited(&visited, nx, ny, dir as u8, GRID_SIZE) {
                            if loop_blocks.insert(simulate_loop_block_pos).is_err() {
                                defmt::info!("Loop block already visited");
                            }else{
                                result_b += 1;
                            }
                            break; 
                        }
                    }
                    if !simulate {
                        update_input_in_place(input, nx, ny, 'X', GRID_SIZE);
                    }
                    x = nx;
                    y = ny;
                }
                if !simulate {
                    cur_pos = (x, y);
                    cur_dir = dir;
                }

                current_step += 1;

            }
        }

        for c in input.chars() {
            if c == 'X' {
                result_a += 1;
            }
        }
        
        //print loop block count
        defmt::info!("Loop block count: {}", result_b);

        // Prepare output
        let mut output = String::<100>::new();

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

fn update_input_in_place(input: &mut String<30000>, x: usize, y: usize, new_char: char, grid_size: usize) {
    assert!(new_char.len_utf8() == 1, "Only single-byte characters are allowed");
    let index = y * (grid_size + 1) + x; // Account for newline
    unsafe {
        let bytes = input.as_bytes_mut(); // Get mutable byte slice
        if bytes[index] != b'\n' {
            bytes[index] = new_char as u8; // Update the character
        } else {
            panic!("Attempted to modify a newline character!");
        }
    }
}

fn encode_position(x: usize, y: usize, dir: u8, width: usize) -> u32 {
    (((y * width + x) << 2) as u32) | (dir as u32)
}

fn mark_visited(visited: &mut [bool], x: usize, y: usize, dir: u8, width: usize) {
    let encoded = encode_position(x, y, dir, width);
    visited[encoded as usize] = true;
}

fn is_visited(visited: &[bool], x: usize, y: usize, dir: u8, width: usize) -> bool {
    let encoded = encode_position(x, y, dir, width);
    visited[encoded as usize]
}

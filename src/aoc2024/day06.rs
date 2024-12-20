use super::Solver;
use core::fmt::Write;
use heapless::String;

pub struct Day06;

impl Solver for Day06 {
    fn solve(input: &String<20000>) -> String<100> {
        const GRID_SIZE: usize = 130;
        const MAX_VISITS: usize = 16900 * 4;

        let mut visited = [false; MAX_VISITS];
        let mut grid = [b'.'; GRID_SIZE * GRID_SIZE];
        let mut start_pos = (0, 0);
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        // Parse the grid
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '^' {
                    start_pos = (x, y);
                    grid[y * GRID_SIZE + x] = b'.';
                } else {
                    grid[y * GRID_SIZE + x] = c as u8;
                }
            }
        }
        #[allow(unused_assignments)]
        let mut result_a = 0;
        let mut result_b = 0;
        
        let mut simulate = false; 
        let mut sim_start_step = 0;
        
        loop {
            let mut current_step = 0;
            if simulate {
                visited.fill(false); 
                simulate = false;
            }else{
                result_a = count_visited(&visited, GRID_SIZE); 
                break;
            }

            let mut x = start_pos.0;
            let mut y = start_pos.1;
            let mut dir = 0;

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
                    //set the nxny to #
                    grid[ny * GRID_SIZE + nx] = b'#';
                    sim_start_step += 1;                    
                }

                let next_idx = ny * GRID_SIZE + nx;

                if grid[next_idx] == b'#' {
                    dir = (dir + 1) % 4;
                } else {
                    if simulate {
                        if is_visited(&visited, nx, ny, dir as u8, GRID_SIZE) {
                            result_b += 1;
                            break; 
                        }
                    }

                    x = nx;
                    y = ny;
                }

                current_step += 1;

            }
        }

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


fn encode_position(x: usize, y: usize, dir: u8, width: usize) -> u32 {
    (((y * width + x) << 2) as u32) | (dir as u32)
}

fn count_visited(visited: &[bool], width: usize) -> usize {
    let mut count = 0;

    for y in 0..width {
        for x in 0..width {
            let base_idx = (y * width + x) * 4;

            if visited[base_idx]
                || visited[base_idx + 1]
                || visited[base_idx + 2]
                || visited[base_idx + 3]
            {
                count += 1;
            }
        }
    }

    count
}

fn mark_visited(visited: &mut [bool], x: usize, y: usize, dir: u8, width: usize) {
    let encoded = encode_position(x, y, dir, width);
    visited[encoded as usize] = true;
}

fn is_visited(visited: &[bool], x: usize, y: usize, dir: u8, width: usize) -> bool {
    let encoded = encode_position(x, y, dir, width);
    visited[encoded as usize]
}

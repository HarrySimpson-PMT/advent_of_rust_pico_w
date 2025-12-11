use crate::solver::Solver;
use heapless::{Deque, String};

pub struct Day;

const N: Neighbors = 0b00000001; // North
const NE: Neighbors = 0b00000010;
const E: Neighbors = 0b00000100;
const SE: Neighbors = 0b00001000;
const S: Neighbors = 0b00010000;
const SW: Neighbors = 0b00100000;
const W: Neighbors = 0b01000000;
const NW: Neighbors = 0b10000000;

type Neighbors = u8;

const DIR_BITS: [(isize, isize, Neighbors); 8] = [
    (-1, -1, NW), // (-1,-1)
    (-1, 0, N),   // (-1, 0)
    (-1, 1, NE),  // (-1, 1)
    (0, -1, W),   // ( 0,-1)
    (0, 1, E),    // ( 0, 1)
    (1, -1, SW),  // ( 1,-1)
    (1, 0, S),    // ( 1, 0)
    (1, 1, SE),   // ( 1, 1)
];

type Cell = u16;

fn load_cell(loc: Cell, input: &String<30000>) -> char {
    input.as_bytes()[loc as usize] as char
}

#[inline(always)]
fn make_cell(x: u8, y: u8, grid_size: u8) -> Cell {
    (y as u16) * (grid_size as u16 + 1) + (x as u16)
}
fn from_cell(cell: Cell, grid_size: u8) -> (u8, u8) {
    let y = (cell / (grid_size as u16 + 1)) as u8;
    let x = (cell % (grid_size as u16 + 1)) as u8;
    (x, y)
}
fn count_neighbors(cell: Cell, input: &String<30000>, grid_size: u8) -> Neighbors {
    let (x, y) = from_cell(cell, grid_size);
    let mut count: Neighbors = 0;

    for (dx, dy, _bit) in DIR_BITS.iter() {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && nx < grid_size as isize && ny >= 0 && ny < grid_size as isize {
            let nx = nx as u8;
            let ny = ny as u8;
            let neighbor_cell = make_cell(nx, ny, grid_size);
            let c = load_cell(neighbor_cell, input);

            if c == '@' {
                count += 1;
            }
        }
    }
    count
}
fn delete_cell(cell: Cell, input: &mut String<30000>) {
    unsafe {
        input.as_bytes_mut()[cell as usize] = b'.';
    }
}
fn delete_cell_safe(cell: Cell, input: String<30000>) -> String<30000> {
    let mut bytes = input.into_bytes();
    bytes[cell as usize] = b'.';
    String::from_utf8(bytes).unwrap_or_default()
}

impl Solver for Day {
    fn solve(input: String<30000>) -> String<100> {
        let mut input = input;
        let mut output = String::<100>::new();
        let mut result_a: u64 = 0;
        let mut result_b: u64 = 0;
        let first_newline = input.find('\n').expect("valid input");
        let grid_size = first_newline as u8; // assumes no \r

        let mut enqueue_count: u64 = 0;
        let mut queue = Deque::<Cell, 5000>::new();

        for y in 0..grid_size {
            for x in 0..grid_size {
                let cell = make_cell(x as u8, y as u8, grid_size as u8);
                let c = load_cell(cell, &input);
                if c == '@' {
                    let neighbors = count_neighbors(cell, &input, grid_size as u8);
                    if neighbors < 4 {
                        result_a += 1;
                        enqueue_count += 1;
                        let res = queue.push_back(cell);
                        if res.is_err() {
                            defmt::error!("Queue overflow when adding cell at ({}, {})", x, y);
                        }
                    }
                }
            }
        }
        while let Some(cell) = queue.pop_front() {
            let (x, y) = from_cell(cell, grid_size as u8);
            if load_cell(cell, &input) != '@' {
                continue;
            }
            delete_cell(cell, &mut input);
            result_b += 1;

            for (dx, dy, _bit) in DIR_BITS.iter() {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < grid_size as isize && ny >= 0 && ny < grid_size as isize {
                    let nx = nx as u8;
                    let ny = ny as u8;
                    let neighbor_cell = make_cell(nx, ny, grid_size as u8);
                    let c = load_cell(neighbor_cell, &input);
                    if c == '@' {
                        let neighbors = count_neighbors(neighbor_cell, &input, grid_size as u8);
                        if neighbors < 4 {
                            enqueue_count += 1;
                            queue.push_back(neighbor_cell).ok();
                        }
                    }
                }
            }
        }

        //iterate through the input and see if there are any more '@' with less than 4 neighbors
        for y in 0..grid_size {
            for x in 0..grid_size {
                let cell = make_cell(x as u8, y as u8, grid_size as u8);
                let c = load_cell(cell, &input);
                if c == '@' {
                    let neighbors = count_neighbors(cell, &input, grid_size as u8);
                    if neighbors < 4 {
                        defmt::info!("Missed cell at ({}, {}) with {} neighbors", x, y, neighbors);
                        result_b += 1;
                    }
                }
            }
        }

        defmt::info!("Enqueued count: {}", enqueue_count);

        //print map
        // for y in 0..grid_size {
        //     let mut line = String::<150>::new();
        //     for x in 0..grid_size {
        //         let cell = make_cell(x as u8, y as u8, grid_size as u8);
        //         let c = load_cell(cell, &input);
        //         line.push(c).ok();
        //     }
        //     defmt::info!("{}", line);
        // }
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

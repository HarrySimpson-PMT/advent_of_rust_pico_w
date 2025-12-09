use crate::solver::Solver;
use heapless::{Deque, FnvIndexMap, String, Vec};

pub struct Day;

impl Solver for Day {
    fn solve(input: String<30000>) -> String<100> {
        let mut output = String::<100>::new();
        let mut result_b: u64 = 0;
        let mut grid: Vec<Vec<char, 137>, 137> = Vec::new();
        let lines: Vec<&str, 137> = input.split('\n').collect();

        for i in 0..lines.len() {
            grid.push(Vec::new()).ok();
            for j in 0..lines[i].len() {
                let _ = grid[i].push(lines[i].chars().nth(j).unwrap());
            }
        }
        let mut dag: FnvIndexMap<(usize, usize), Vec<(usize, usize), 9>, 16384> = FnvIndexMap::new();
        let mut queue: Deque<(usize, usize), 8000> = Deque::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let mut neighbors: Vec<(usize, usize), 9> = Vec::new();
                for di in -1i32..=1 {
                    for dj in -1i32..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i as i32 + di;
                        let nj = j as i32 + dj;
                        if ni >= 0 && ni < grid.len() as i32 && nj >= 0 && nj < grid[i].len() as i32
                        {
                            let ni = ni as usize;
                            let nj = nj as usize;
                            if grid[ni][nj] == '@' {
                                neighbors.push((ni, nj)).ok();
                            }
                        }
                    }
                }
                if neighbors.len() < 4 {
                    queue.push_back((i, j)).ok();
                }
                dag.insert((i, j), neighbors).ok();
            }
        }
        let result_a = queue.len() as u64;
        while let Some(node @ (i, j)) = queue.pop_front() {
            let neighbors = dag.remove(&node).unwrap();
            result_b += 1;
            for neigh in neighbors {
                if let Some(neighbor) = dag.remove(&neigh) {
                    let mut new_neighbors: Vec<(usize, usize), 9> = Vec::new();
                    for n in neighbor {
                        if n != node {
                            new_neighbors.push(n).ok();
                        }
                    }
                    if new_neighbors.len() <4 {
                        queue.push_back(neigh).ok();
                    }
                    dag.insert(neigh, new_neighbors).ok();
                }
            }
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

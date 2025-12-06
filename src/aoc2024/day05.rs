use crate::solver::Solver;
use core::fmt::Write;
use heapless::{FnvIndexMap, String, Vec};

pub struct Day;

impl Solver for Day {
    fn solve(input: &mut String<30000>) -> String<100> {
        let mut output = String::<100>::new();
        let mut first_result: String<50> = String::new();
        let mut second_result: String<50> = String::new();

        defmt::info!("Starting Day05 Solver");

        if input.is_empty() {
            output.push_str("Error: Empty input").ok();
            return output;
        }

        let mut dictionary = FnvIndexMap::<u8, Vec<u8, 24>, 2048>::new();
        let mut valid_lines = Vec::<Vec<u8, 26>, 512>::new();
        let mut invalid_lines = Vec::<Vec<u8, 26>, 512>::new();

        let mut is_second_part = false;
        let mut current_line_start = 0;

        for (i, c) in input.char_indices() {
            if c == '\n' {
                if i > 0 && &input[i - 1..=i] == "\n\n" {
                    defmt::info!("Switching to second part");
                    is_second_part = true;
                    current_line_start = i + 1;
                    continue;
                }

                let line = &input[current_line_start..i];
                current_line_start = i + 1;

                if !is_second_part {
                    if let Some((left, right)) = line.split_once('|') {
                        if let (Ok(x), Ok(y)) =
                            (left.trim().parse::<u8>(), right.trim().parse::<u8>())
                        {
                            if let Some(values) = dictionary.get_mut(&y) {
                                if values.len() < 24 {
                                    values.push(x).ok();
                                }
                                else {
                                    defmt::error!("Dictionary value too long");
                                }
                            } else {
                                let mut new_vec = Vec::<u8, 24>::new();
                                new_vec.push(x).ok();
                                dictionary.insert(y, new_vec).ok();
                            }
                        }
                    }
                } else {
                    let mut numbers = Vec::<u8, 26>::new();
                    for num in line.split(',') {
                        if let Ok(value) = num.trim().parse::<u8>() {
                            if numbers.len() < 26 {
                                numbers.push(value).ok();
                            }
                        }
                    }
                    if check_validity(&numbers, &dictionary) {
                        valid_lines.push(numbers).ok();
                    } else {
                        invalid_lines.push(numbers).ok();
                    }
                }
            }
        }        
        if current_line_start < input.len() {
            let line = &input[current_line_start..];
            if is_second_part {
                let mut numbers = Vec::<u8, 26>::new();
                for num in line.split(',') {
                    if let Ok(value) = num.trim().parse::<u8>() {
                        if numbers.len() < 26 {
                            numbers.push(value).ok();
                        }
                    }
                }                
                if check_validity(&numbers, &dictionary) {
                    valid_lines.push(numbers).ok();
                } else {
                    invalid_lines.push(numbers).ok();
                }
            }
        }
        defmt::info!("data parsed - lines: valid: {} invalid {}", valid_lines.len(), invalid_lines.len());

        let result_a: u32 = valid_lines
            .iter()
            .map(|line| calculate_middle(line) as u32) 
            .sum();

        let result_b = fix_invalid_lines(&invalid_lines, &dictionary);

        write!(first_result, "{}", result_a).ok();
        write!(second_result, "{}", result_b).ok();

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

fn check_validity(numbers: &[u8], dictionary: &FnvIndexMap<u8, Vec<u8, 24>, 2048>) -> bool {
    let mut visited: Vec<u8, 26> = Vec::new();
    let mut not_visited: Vec<u8, 26> = numbers.iter().cloned().collect();

    for &num in numbers {
        if let Some(entry) = dictionary.get(&num) {
            let pre_ok = entry.iter().all(|&pre_key| !not_visited.contains(&pre_key));
            if !pre_ok {
                return false;
            }
        }
        visited.push(num).ok();
        not_visited.retain(|&x| x != num);
    }
    true
}

fn calculate_middle(numbers: &[u8]) -> u8 {
    if numbers.is_empty() {
        0
    } else {
        let middle_index = numbers.len() / 2;
        numbers[middle_index]
    }
}

fn fix_invalid_lines(
    invalid_lines: &[Vec<u8, 26>],
    dictionary: &FnvIndexMap<u8, Vec<u8, 24>, 2048>,
) -> u32 {
    let mut results = 0;

    for numbers in invalid_lines {
        let corrected_line = topological_sort_line(numbers, dictionary);
        if !corrected_line.is_empty() {
            let middle_index = corrected_line.len() / 2;
            results += corrected_line[middle_index] as u32;
        }
    }

    results
}

fn topological_sort_line(
    numbers: &[u8],
    dictionary: &FnvIndexMap<u8, Vec<u8, 24>, 2048>,
) -> Vec<u8, 24> {
    let mut in_degree: FnvIndexMap<u8, usize, 2048> = FnvIndexMap::new();
    let mut graph: FnvIndexMap<u8, Vec<u8, 24>, 2048> = FnvIndexMap::new();

    for &num in numbers {
        if let Some(dependencies) = dictionary.get(&num) {
            if !graph.contains_key(&num) {
                graph.insert(num, Vec::new()).ok();
            }
            graph
                .get_mut(&num)
                .unwrap()
                .extend(dependencies.iter().cloned());

            if !in_degree.contains_key(&num) {
                in_degree.insert(num, 0).ok();
            }

            for &dep in dependencies {
                if numbers.contains(&dep) {
                    if !in_degree.contains_key(&dep) {
                        in_degree.insert(dep, 0).ok();
                    }
                    *in_degree.get_mut(&dep).unwrap() += 1;
                }
            }
        }
    }

    let mut queue: Vec<u8, 16> = in_degree
        .iter()
        .filter(|&(_, &degree)| degree == 0)
        .map(|(&node, _)| node)
        .collect();

    let mut sorted_order = Vec::new();

    while let Some(node) = queue.pop() {
        sorted_order.push(node).ok();
        if let Some(edges) = graph.remove(&node) {
            for edge in edges {
                if let Some(degree) = in_degree.get_mut(&edge) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push(edge).ok();
                    }
                }
            }
        }
    }

    sorted_order
}

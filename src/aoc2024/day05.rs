use super::Solver;
use core::{fmt::Write, str::FromStr};
use heapless::{FnvIndexMap, String, Vec};

pub struct Day05;

impl Solver for Day05 {
    fn solve(input: &String<20000>) -> String<100> {
        let mut output = String::<100>::new();
        let mut first_result: String<50> = String::new();
        let mut second_result: String<50> = String::new();

        defmt::info!("test");

        if input.is_empty() {
            output.push_str("Error: Empty input").ok();
            return output;
        }

        let mut lines: Vec<String<100>, 1400> = Vec::new();
        for line in input.lines() {
            lines
                .push(String::<100>::from_str(line).unwrap_or_else(|_| String::new()))
                .ok();
        }

        // Split into two parts
        let mut iter = lines.split(|line| line.is_empty());
        let first_part: Vec<String<5>, 1200> = iter
            .next()
            .unwrap()
            .iter()
            .map(|line| {
                let trimmed = &line[..line.len().min(5)]; // Safely trim to 5 characters
                String::<5>::from_str(trimmed).unwrap()
            })
            .collect();

        let second_part: Vec<String<100>, 400> = iter.next().unwrap().iter().cloned().collect();

        // Populate dictionary using FnvIndexMap
        let dictionary = populate_dictionary(&first_part);

        // Part A: Calculate results from input
        let result_a = calculate_results_from_input(&second_part, &dictionary);

        // Part B: Fix invalid lines and calculate results
        let (_, invalid_lines) = split_lines(&second_part, &dictionary);
        let result_b = fix_invalid_lines(&invalid_lines, &dictionary);

        // Write results into output
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

fn populate_dictionary(input: &[String<5>]) -> FnvIndexMap<i32, Vec<i32, 24>, 2048> {
    let mut dictionary = FnvIndexMap::<i32, Vec<i32, 24>, 2048>::new();

    for line in input {
        if let Some((x_str, y_str)) = line.split_once('|') {
            if let (Ok(x), Ok(y)) = (x_str.trim().parse(), y_str.trim().parse()) {
                if let Some(values) = dictionary.get_mut(&y) {
                    values.push(x).ok();
                } else {
                    let mut new_vec = Vec::<i32, 24>::new();
                    new_vec.push(x).ok();
                    dictionary.insert(y, new_vec).ok();
                }
            }
        }
    }

    dictionary
}


// Results calculation
fn calculate_results_from_input(
    second_part: &[String<100>],
    dictionary: &FnvIndexMap<i32, Vec<i32, 24>, 2048>,
) -> i32 {
    let (valid_lines, _) = split_lines(second_part, dictionary);

    valid_lines.iter().map(|line| calculate_middle(line)).sum()
}

// Line splitting
fn split_lines(
    second_part: &[String<100>],
    dictionary: &FnvIndexMap<i32, Vec<i32, 24>, 2048>,
) -> (Vec<Vec<i32, 24>, 2048>, Vec<Vec<i32, 24>, 2048>) {
    let mut valid_lines = Vec::new();
    let mut invalid_lines = Vec::new();

    for line in second_part {
        let numbers: Vec<i32, 24> = line
            .split(',')
            .filter_map(|num| num.trim().parse::<i32>().ok())
            .collect();

        if check_validity(&numbers, dictionary) {
            valid_lines.push(numbers).ok();
        } else {
            invalid_lines.push(numbers).ok();
        }
    }

    (valid_lines, invalid_lines)
}

// Validity checking
fn check_validity(numbers: &[i32], dictionary: &FnvIndexMap<i32, Vec<i32, 24>, 2048>) -> bool {
    let mut visited: Vec<i32, 16> = Vec::new();
    let mut not_visited: Vec<i32, 16> = numbers.iter().cloned().collect();

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

// Middle value calculation
fn calculate_middle(numbers: &[i32]) -> i32 {
    if numbers.is_empty() {
        0
    } else {
        let middle_index = numbers.len() / 2;
        numbers[middle_index]
    }
}

// Fix invalid lines
fn fix_invalid_lines(
    invalid_lines: &[Vec<i32, 24>],
    dictionary: &FnvIndexMap<i32, Vec<i32, 24>, 2048>,
) -> i32 {
    let mut results = 0;

    for numbers in invalid_lines {
        let corrected_line = topological_sort_line(numbers, dictionary);

        if !corrected_line.is_empty() {
            let middle_index = corrected_line.len() / 2;
            results += corrected_line[middle_index];
        }
    }

    results
}

// Topological sort for invalid lines
fn topological_sort_line(
    numbers: &[i32],
    dictionary: &FnvIndexMap<i32, Vec<i32, 24>, 2048>,
) -> Vec<i32, 16> {
    let mut in_degree: FnvIndexMap<i32, usize, 2048> = FnvIndexMap::new();
    let mut graph: FnvIndexMap<i32, Vec<i32, 24>, 2048> = FnvIndexMap::new();

    for &num in numbers {
        if let Some(dependencies) = dictionary.get(&num) {
            if !graph.contains_key(&num) {
                graph.insert(num, Vec::new());
            }
            graph.get_mut(&num).unwrap().extend(dependencies.iter().cloned());
    
            if !in_degree.contains_key(&num) {
                in_degree.insert(num, 0);
            }
    
            for &dep in dependencies {
                if numbers.contains(&dep) {
                    if !in_degree.contains_key(&dep) {
                        in_degree.insert(dep, 0);
                    }
                    *in_degree.get_mut(&dep).unwrap() += 1;
                }
            }
        }
    }    

    let mut queue: Vec<i32, 16> = in_degree
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

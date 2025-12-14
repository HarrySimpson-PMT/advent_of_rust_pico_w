use crate::solver::Solver;
use core::fmt::Write;
use heapless::{String, Vec};

pub struct Day;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
    len_sq: i64, // squared length — exact, sortable, no overflow
}
impl Solver for Day {
    fn solve(input: String<30000>) -> String<100> {
        defmt::info!("Solving Day x...");
        let mut output = String::<100>::new();
        let mut result_a: u64 = 0;
        let lines: Vec<&str, 1000> = input.split('\n').collect();

        let mut lines_vec: Vec<Line, 1000> = Vec::new();
        let mut points_vec: Vec<Point, 1000> = Vec::new();
        // create all the points
        for line in &lines {
            let parts: Vec<&str, 2> = line.split(',').collect();
            let point = Point {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
            };
            points_vec.push(point).unwrap();
        }

         for i in 0..points_vec.len()-1 {
            for j in i + 1..points_vec.len() {
                let pointa: Point = {
                    let parts: Vec<&str, 3> = lines[i].split(',').collect();
                    Point {
                        x: parts[0].parse().unwrap(),
                        y: parts[1].parse().unwrap(),
                    }
                };
                let pointb: Point = {
                    let parts: Vec<&str, 3> = lines[j].split(',').collect();
                    Point {
                        x: parts[0].parse().unwrap(),
                        y: parts[1].parse().unwrap(),
                    }
                };
                // let area: i64 = ((pointb.x - pointa.x).abs() +1) * ((pointb.y - pointa.y).abs() +1) as i64;
                let area: i64 = ((pointb.x - pointa.x).abs() as i64 + 1)
                    * ((pointb.y - pointa.y).abs() as i64 + 1);
                if result_a < area as u64 {
                    result_a = area as u64;
                }
            }
        }

        // create all teh lines
        for i in 0..lines.len() - 1 {
            let start_point: Point = {
                let parts: Vec<&str, 2> = lines[i].split(',').collect();
                Point {
                    x: parts[0].parse().unwrap(),
                    y: parts[1].parse().unwrap(),
                }
            };
            let end_point: Point = {
                let parts: Vec<&str, 2> = lines[i + 1].split(',').collect();
                Point {
                    x: parts[0].parse().unwrap(),
                    y: parts[1].parse().unwrap(),
                }
            };
            let dx = end_point.x as i64 - start_point.x as i64;
            let dy = end_point.y as i64 - start_point.y as i64;
            let len_sq = dx * dx + dy * dy;

            let _ = lines_vec.push(Line {
                start: start_point,
                end: end_point,
                len_sq,
            });
        }
        
       
        let first_point: Point = {
            let parts: Vec<&str, 2> = lines[0].split(',').collect();
            Point {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
            }
        };
        let last_point: Point = {
            let parts: Vec<&str, 2> = lines[lines.len() - 1].split(',').collect();
            Point {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
            }
        };
        let dx = last_point.x as i64 - first_point.x as i64;
        let dy = last_point.y as i64 - first_point.y as i64;
        let len_sq = dx * dx + dy * dy;
        let _ = lines_vec.push(Line {
            start: last_point,
            end: first_point,
            len_sq,
        });
        // let (min_x, max_x, min_y, max_y) = build_boundary(&lines_vec);

        // lines_vec.sort_by(|a, b| b.len_sq.cmp(&a.len_sq));
        lines_vec.sort_unstable_by(|a, b| b.len_sq.cmp(&a.len_sq));
        let big_north_line = &lines_vec[1];
        let north_right_point = if big_north_line.start.x > big_north_line.end.x {
            big_north_line.start
        } else {
            big_north_line.end
        };
        let big_south_line = &lines_vec[0];
        let south_right_point = if big_south_line.start.x > big_south_line.end.x {
            big_south_line.start
        } else {
            big_south_line.end
        };

        let points_north_west = points_vec
            .iter()
            .filter(|p| p.x < north_right_point.x)
            .filter(|p| p.y > north_right_point.y)
            .collect::<Vec<&Point, 500>>();
        let points_south_west = points_vec
            .iter()
            .filter(|p| p.x < south_right_point.x)
            .filter(|p| p.y < south_right_point.y)
            .collect::<Vec<&Point, 500>>();

      
        let mut largest_area: u64 = 0;
        for p in &points_north_west {
            let mut is_valid = true;
            //check if valid ie no other points inside the rectangle
            for op in &points_north_west {
                if **op == **p {
                    continue;
                }
                if op.x >= p.x
                    && op.x <= north_right_point.x
                    && op.y <= p.y
                    && op.y >= north_right_point.y
                {
                    is_valid = false;
                    break;
                }
            }
            if !is_valid {
                continue;
            }

            let current_area: u64 = ((north_right_point.x - p.x).abs() as u64 + 1)
                * ((north_right_point.y - p.y).abs() as u64 + 1);
            if current_area > largest_area {
                largest_area = current_area;
            }
        }
        //draw larger north west rectangle
        let mut result_b = largest_area;

        let mut largest_area: u64 = 0;
        for p in &points_south_west {
            let mut is_valid = true;
            //check if valid ie no other points inside the rectangle
            for op in &points_south_west {
                if **op == **p {
                    continue;
                }
                if op.x >= p.x
                    && op.x <= south_right_point.x
                    && op.y >= p.y
                    && op.y <= south_right_point.y
                {
                    is_valid = false;
                    break;
                }
            }
            if !is_valid {
                continue;
            }
            let current_area: u64 = ((south_right_point.x - p.x).abs() as u64 + 1)
                * ((south_right_point.y - p.y).abs() as u64 + 1);
            if current_area > largest_area {
                largest_area = current_area;
            }
        }

        if largest_area > result_b {
            result_b = largest_area;
        }

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

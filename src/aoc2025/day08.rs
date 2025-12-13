use crate::solver::Solver;
use core::fmt::Write;
use heapless::{BinaryHeap, String, Vec};

pub struct Day;

struct Space3D {
    id: u16,
    x: i32,
    y: i32,
    z: i32,
}

impl Solver for Day {
    fn solve(input: String<30000>) -> String<100> {
        let mut output = String::<100>::new();
        let mut result_a: u64 = 0;
        let mut result_b: u64 = 0;
        let mut heap_max: BinaryHeap<(u32, u16, u16), heapless::binary_heap::Max, 6000> =
            BinaryHeap::new();
        let mut sorted_edges: Vec<((u16, u16), u32), 6000> = Vec::new();
        let mut points: Vec<Space3D, 1001> = Vec::new();
        defmt::info!("Parsing input...");
        let lines: Vec<&str, 1000> = input.split('\n').collect();
        for i in 0..lines.len() {
            let line = lines[i];
            let mut fields = line.split(',');
            let coord_1 = match fields.next().and_then(|s| s.parse::<i32>().ok()) {
                Some(v) => v,
                None => continue, // skip line
            };
            let coord_2 = match fields.next().and_then(|s| s.parse::<i32>().ok()) {
                Some(v) => v,
                None => continue,
            };
            let coord_3 = match fields.next().and_then(|s| s.parse::<i32>().ok()) {
                Some(v) => v,
                None => continue,
            };

            let _ = points.push(Space3D {
                id: i as u16,
                x: coord_1,
                y: coord_2,
                z: coord_3,
            });
        }
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let dx: i64 = points[i].x as i64 - points[j].x as i64;
                let dy: i64 = points[i].y as i64 - points[j].y as i64;
                let dz: i64 = points[i].z as i64 - points[j].z as i64;
                let dist = (dx * dx + dy * dy + dz * dz) as i64;
                if dist > u32::MAX as i64 {
                    continue;
                }
                if heap_max.len() < 6000 {
                    let _ = heap_max.push((dist as u32, i as u16, j as u16));
                } else if let Some(&(max_dist, _, _)) = heap_max.peek() {
                    if (dist as u32) < max_dist {
                        let _ = heap_max.pop();
                        let _ = heap_max.push((dist as u32, i as u16, j as u16));
                    }
                }
            }
            if i % 100 == 0 {
                defmt::info!("Processed point {} of {}", i, points.len());
            }
        }
        let mut parent: Vec<u16, 1001> = (0..points.len() as u16).collect();
        fn find(parent: &mut Vec<u16, 1001>, x: u16) -> u16 {
            if parent[x as usize] != x {
                parent[x as usize] = find(parent, parent[x as usize]);
            }
            parent[x as usize]
        }
        fn union(parent: &mut Vec<u16, 1001>, x: u16, y: u16) {
            let root_x = find(parent, x);
            let root_y = find(parent, y);
            if root_x != root_y {
                parent[root_y as usize] = root_x;
            }
        }
        while let Some((neg_dist, i, j)) = heap_max.pop() {
            sorted_edges
                .push(((i, j), neg_dist))
                .unwrap_or_else(|_| panic!("Failed to push edge to sorted_edges"));
        }

        defmt::info!("Processing edges for MST...");
        let mut target_connections = 1001;
        for &((i, j), neg_dist) in sorted_edges.iter().rev() {
            target_connections -= 1;
            defmt::info!("Target connections remaining: {}", target_connections);
            if find(&mut parent, i) == find(&mut parent, j) {
                continue;
            }
            union(&mut parent, i, j);
            let mut unique_parents: heapless::FnvIndexMap<usize, usize, 1024> =
                heapless::FnvIndexMap::new();
            for k in 0..points.len() {
                let root = find(&mut parent, k as u16) as usize;
                if unique_parents.contains_key(&root) {
                    let count = unique_parents.get_mut(&root).unwrap();
                    *count += 1;
                } else {
                    let _ = unique_parents.insert(root, 1);
                }
            }
            if unique_parents.len() <= 1 {
                let product = points[i as usize].x * points[j as usize].x;
                defmt::info!(
                    "Last connection between point {} and point {} with distance {}. Product of x coords: {}",
                    i,
                    j,
                    neg_dist,
                    product
                );
                result_b = product as u64;
                break;
            }
            if target_connections == 0 {             
                let mut sizes: Vec<usize, 1024> = Vec::new();
                for (_root, &count) in unique_parents.iter() {
                    let _ = sizes.push(count);
                }

                sizes.sort_unstable_by(|a, b| b.cmp(a));

                if sizes.len() >= 3 {
                    let top_three_product = sizes[0] * sizes[1] * sizes[2];

                    defmt::info!("Found groups: {:?}, product: {}", sizes, top_three_product);
                    result_a = top_three_product as u64;
                }

                target_connections = -1; // Stop checking
            }
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

use std::collections::HashMap;
use std::ops::Not;

advent_of_code::solution!(11);

const START: &str = "you";
const END: &str = "out";
const SVR: &str = "svr";
const DAC: &str = "dac";
const FFT: &str = "fft";

fn dfs<'a>(graph: &HashMap<&str, Vec<&'a str>>, total_ways: &mut u64, start: &'a str) {
    for neighbor in graph[start].iter() {
        if *neighbor == END {
            *total_ways += 1;
            return;
        } else {
            dfs(graph, total_ways, neighbor);
        }
    }
}

fn sort_topologically<'a>(
    graph: &HashMap<&str, Vec<&'a str>>,
    topologically_sorted: &mut Vec<&'a str>,
    visited: &mut HashMap<&'a str, bool>,
    start: &'a str,
) {
    visited.insert(start, true);
    for neighbor in graph.get(start).into_iter().flatten() {
        if visited.contains_key(neighbor).not() {
            sort_topologically(graph, topologically_sorted, visited, neighbor);
        }
    }
    // push after exploring children so reversing gives a proper topological order
    topologically_sorted.push(start);
}

fn get_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (start, edges) = line.split_once(":").unwrap();
        for edge in edges.trim().split(' ') {
            if graph.contains_key(start) {
                graph.get_mut(start).unwrap().push(edge);
            } else {
                graph.insert(start, vec![edge]);
            }
        }
    }
    graph
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = get_graph(input);
    let mut total_ways: u64 = 0;
    dfs(&graph, &mut total_ways, START);
    Some(total_ways)
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = get_graph(input);
    let mut topologically_sorted = Vec::new();
    sort_topologically(&graph, &mut topologically_sorted, &mut HashMap::new(), SVR);
    topologically_sorted.reverse();

    // c0: seen neither, cD: seen dac, cF: seen fft, cB: seen both
    let mut c0 = HashMap::new();
    let mut c_b = HashMap::new();
    let mut c_f = HashMap::new();
    let mut c_d = HashMap::new();

    c0.insert(SVR, 1);
    c_b.insert(SVR, 0);
    c_f.insert(SVR, 0);
    c_d.insert(SVR, 0);

    for node in topologically_sorted {
        let c0u = *c0.get(node).unwrap_or(&0u64);
        let c_du = *c_d.get(node).unwrap_or(&0u64);
        let c_fu = *c_f.get(node).unwrap_or(&0u64);
        let c_bu = *c_b.get(node).unwrap_or(&0u64);

        for neighbor in graph.get(node).into_iter().flatten() {
            match *neighbor {
                FFT => {
                    let new_f = c_f.get(neighbor).unwrap_or(&0u64) + c_fu + c0u;
                    let new_b = c_b.get(neighbor).unwrap_or(&0u64) + c_du + c_bu;
                    c_f.insert(neighbor, new_f);
                    c_b.insert(neighbor, new_b);
                }
                DAC => {
                    let new_d = c_d.get(neighbor).unwrap_or(&0u64) + c_du + c0u;
                    let new_b = c_b.get(neighbor).unwrap_or(&0u64) + c_fu + c_bu;
                    c_d.insert(neighbor, new_d);
                    c_b.insert(neighbor, new_b);
                }
                &_ => {
                    c0.insert(neighbor, c0.get(neighbor).unwrap_or(&0u64) + c0u);
                    c_d.insert(neighbor, c_d.get(neighbor).unwrap_or(&0u64) + c_du);
                    c_f.insert(neighbor, c_f.get(neighbor).unwrap_or(&0u64) + c_fu);
                    c_b.insert(neighbor, c_b.get(neighbor).unwrap_or(&0u64) + c_bu);
                }
            }
        }
    }
    Some(*c_b.get(END).unwrap_or(&0u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}

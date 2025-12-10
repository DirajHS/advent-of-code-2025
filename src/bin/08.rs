use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

advent_of_code::solution!(8);

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, PartialEq, Eq)]
struct Circuit {
    distance: isize,
    point1: Point,
    point2: Point,
}

impl Ord for Circuit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialOrd for Circuit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn euclidean_distance(p1: Point, p2: Point) -> isize {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let dz = p1.z - p2.z;
    let sum = dx * dx + dy * dy + dz * dz;
    sum.isqrt()
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    comps: usize,
}

impl Dsu {
    fn new(items: usize) -> Self {
        Self {
            parent: (0..items).collect(),
            size: vec![1; items],
            comps: items,
        }
    }
    fn find(&mut self, item: usize) -> usize {
        if self.parent[item] == item {
            return self.parent[item];
        } else {
            self.parent[item] = self.find(self.parent[item])
        }
        self.parent[item]
    }

    fn union(&mut self, item1: usize, item2: usize) -> bool {
        let (mut parent_1, mut parent_2) = (self.find(item1), self.find(item2));
        if parent_1 == parent_2 {
            return false;
        }
        if self.size[parent_1] < self.size[parent_2] {
            std::mem::swap(&mut parent_1, &mut parent_2);
        }
        self.parent[parent_2] = parent_1;
        self.size[parent_1] += self.size[parent_2];
        self.comps -= 1;
        true
    }

    fn compute_size(&mut self, item: usize) -> usize {
        let parent = self.find(item);
        self.size[parent]
    }

    fn are_all_components_connected(&mut self) -> bool {
        self.comps == 1
    }
}

fn parse_input(input: &str) -> (BinaryHeap<Circuit>, Vec<Point>, HashMap<Point, usize>) {
    let mut circuit_min_heap: BinaryHeap<Circuit> = BinaryHeap::new();
    let mut points: Vec<Point> = Vec::new();
    let mut points_mapping: HashMap<Point, usize> = HashMap::new();
    for (idx, line) in input.lines().enumerate() {
        let point_vec = line
            .split(',')
            .map(|p| p.parse().unwrap())
            .collect::<Vec<isize>>();
        let point = Point {
            x: point_vec[0],
            y: point_vec[1],
            z: point_vec[2],
        };
        points.push(point);
        points_mapping.insert(point, idx);
    }
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            circuit_min_heap.push(Circuit {
                distance: euclidean_distance(points[i], points[j]),
                point1: points[i],
                point2: points[j],
            })
        }
    }
    (circuit_min_heap, points, points_mapping)
}
pub fn part_one(input: &str) -> Option<u64> {
    let (mut circuit_min_heap, points, points_mapping) = parse_input(input);
    let mut disjoint_set = Dsu::new(points.len());
    let mut processed_boxes = 0;
    while let Some(circ) = circuit_min_heap.pop()
        && processed_boxes < 10
    // Replace it to 1000 for actual input
    {
        disjoint_set.union(points_mapping[&circ.point1], points_mapping[&circ.point2]);
        processed_boxes += 1;
    }
    let mut junction_box_sizes = Vec::new();
    for point in 0..points.len() {
        if disjoint_set.parent[point] == point {
            junction_box_sizes.push(disjoint_set.compute_size(point));
        }
    }
    junction_box_sizes.sort_by(|a, b| b.partial_cmp(a).unwrap());
    Some(junction_box_sizes[0] as u64 * junction_box_sizes[1] as u64 * junction_box_sizes[2] as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut circuit_min_heap, points, points_mapping) = parse_input(input);
    let mut disjoint_set = Dsu::new(points.len());
    while let Some(circ) = circuit_min_heap.peek() {
        if disjoint_set.union(points_mapping[&circ.point1], points_mapping[&circ.point2])
            && disjoint_set.are_all_components_connected()
        {
            //println!("All components connected");
            break;
        } else {
            circuit_min_heap.pop();
        }
    }
    let last_junction_box = circuit_min_heap.pop().unwrap();
    Some((last_junction_box.point1.x * last_junction_box.point2.x) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}

use std::cmp::max;

advent_of_code::solution!(5);

#[derive(Copy, Clone, Default, Debug)]
struct Interval {
    low: u64,
    high: u64,
}

#[derive(Clone, Default)]
struct Node {
    interval: Interval,
    max: u64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn insert(&mut self, interval: Interval) {
        if interval.low < self.interval.low {
            if let Some(left_child_node) = &mut self.left {
                left_child_node.insert(interval);
            } else {
                self.left = Some(Box::new(Node {
                    interval,
                    max: interval.high,
                    left: None,
                    right: None,
                }));
            }
        } else if let Some(right_child_node) = &mut self.right {
            right_child_node.insert(interval);
        } else {
            self.right = Some(Box::new(Node {
                interval,
                max: interval.high,
                left: None,
                right: None,
            }));
        }
        self.update_max();
    }

    fn update_max(&mut self) {
        let mut current_max = self.max;
        if let Some(left_child_node) = &mut self.left {
            current_max = current_max.max(left_child_node.max);
        }
        if let Some(right_child_node) = &mut self.right {
            current_max = current_max.max(right_child_node.max);
        }
        self.max = current_max;
    }

    fn is_overlapping(interval1: &Interval, interval2: &Interval) -> bool {
        if interval2.low <= interval1.high && interval1.low <= interval2.high {
            return true;
        }
        false
    }

    fn search(&self, interval: &Interval) -> Option<bool> {
        if Self::is_overlapping(interval, &self.interval) {
            return Some(true);
        }
        if let Some(left) = &self.left
            && left.max >= interval.low
        {
            return left.search(interval);
        }
        if let Some(right) = &self.right {
            return right.search(interval);
        }
        Some(false)
    }
}

fn merge_intervals(intervals: &mut [Interval]) -> Vec<Interval> {
    intervals.sort_by(|a, b| a.low.cmp(&b.low));

    let mut merged_intervals: Vec<Interval> = Vec::new();
    merged_intervals.push(*intervals.first().unwrap());
    //println!("{:?}", merged_intervals.last().unwrap());

    for interval in intervals.iter().skip(1) {
        let last_interval = merged_intervals.last_mut().unwrap();
        if interval.low <= last_interval.high {
            last_interval.high = max(interval.high, last_interval.high)
        } else {
            merged_intervals.push(*interval);
        }
        //println!("{:?}", merged_intervals.last().unwrap());
    }
    merged_intervals
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut node = Node::default();
    let (ranges_s, ingredients_s) = input.split_once("\n\n").unwrap_or((input, ""));

    ranges_s
        .lines()
        .filter(|l| !l.trim().is_empty())
        .for_each(|line| {
            let (a, b) = line.split_once('-').expect("range a-b");
            let low: u64 = a.trim().parse().unwrap();
            let high: u64 = b.trim().parse().unwrap();
            node.insert(Interval { low, high });
        });

    let count = ingredients_s
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().parse::<u64>().unwrap())
        .filter(|ingredient| {
            node.search(&Interval {
                low: *ingredient,
                high: *ingredient,
            })
            .unwrap()
        })
        .count() as u64;

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges_s, _) = input.split_once("\n\n").unwrap_or((input, ""));

    let mut intervals: Vec<Interval> = ranges_s
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (a, b) = line.split_once('-').expect("range a-b");
            let low: u64 = a.trim().parse().unwrap();
            let high: u64 = b.trim().parse().unwrap();
            Interval { low, high }
        })
        .collect();

    intervals = merge_intervals(&mut intervals);

    let result: u64 = intervals
        .iter()
        .map(|interval| interval.high - interval.low + 1)
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}

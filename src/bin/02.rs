use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalid_ids: u64 = 0;
    input.split(',').for_each(|range| {
        let ids = range
            .split('-')
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        //println!("{} Looping for : {} entries", range, ids[1] - ids[0]);
        for id in ids[0]..=ids[1] {
            let str_num = id.to_string();
            if str_num.len() % 2 == 0 {
                let (left, right) = str_num.split_at(str_num.len() / 2);
                if left.eq(right) {
                    invalid_ids += id;
                }
            }
        }
    });
    Some(invalid_ids)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut invalid_ids: u64 = 0;
    input.split(',').for_each(|range| {
        let ids = range
            .split('-')
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        //println!("{} Looping for : {} entries", range, ids[1] - ids[0]);
        for id in ids[0]..=ids[1] {
            let mut frequency = HashMap::new();
            let mut pattern = String::new();
            let str_num = id.to_string();
            for ch in str_num.chars() {
                if let std::collections::hash_map::Entry::Vacant(e) = frequency.entry(ch) {
                    pattern.push(ch);
                    e.insert(true);
                } else {
                    let pattern_collected: Vec<&str> = str_num.split_inclusive(&pattern).collect();
                    let mut all_pattern_same = true;
                    for p in pattern_collected.iter() {
                        if *p != pattern {
                            all_pattern_same = false;
                            break;
                        }
                    }
                    if all_pattern_same && pattern_collected.len() >= 2 {
                        //println!("{} Pattern found {}, id: {}", range, pattern, id);
                        invalid_ids += id;
                        break;
                    } else {
                        pattern.push(ch);
                    }
                }
            }
        }
    });
    Some(invalid_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}

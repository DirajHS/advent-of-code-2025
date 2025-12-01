advent_of_code::solution!(1);

const RANGE: usize = 100;
pub fn part_one(input: &str) -> Option<u64> {
    let mut dial: usize = 50;
    let mut result: u64 = 0;
    for rotation in input.lines() {
        let (direction, number) = rotation.split_at(1);
        match direction {
            "R" => {
                dial = (dial + number.parse::<usize>().unwrap()) % RANGE;
            }
            "L" => {
                let n = number.parse::<usize>().unwrap();
                dial = (dial + RANGE - (n % RANGE)) % RANGE;
            }
            _ => panic!("Unknown direction: {}", direction),
        }
        if dial == 0 {
            result += 1;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial: usize = 50;
    let mut result: u64 = 0;
    for rotation in input.lines() {
        let (direction, n) = rotation.split_at(1);
        let number = n.parse::<usize>().unwrap();
        match direction {
            "R" => {
                let wraps = (dial + number) / RANGE;
                result += wraps as u64;
                dial = (dial + number) % RANGE;
            }
            "L" => {
                let wraps = (number + (RANGE - dial)) / RANGE - if dial == 0 { 1 } else { 0 };
                result += wraps as u64;
                dial = (dial + RANGE - (number % RANGE)) % RANGE;
            }
            _ => panic!("Unknown direction: {}", direction),
        }
    }
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
        assert_eq!(result, Some(6));
    }
}

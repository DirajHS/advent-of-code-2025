advent_of_code::solution!(3);

fn jolt(bank_line: &str, digits: usize) -> u64 {
    let bank: Vec<u64> = bank_line
        .to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap().into())
        .collect();
    let mut jolt_stack: Vec<u64> = Vec::new();
    let mut removable_digits = bank.len() - digits;
    for digit in &bank {
        while removable_digits > 0 && digit > jolt_stack.last().unwrap_or(&u64::MAX) {
            jolt_stack.pop();
            removable_digits -= 1;
        }
        jolt_stack.push(*digit);
    }
    jolt_stack.truncate(digits);

    let jolt: u64 = jolt_stack.iter().fold(0, |acc, x| (acc * 10) + (*x));
    //println!("jolt: {}", jolt);
    jolt
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    for line in input.lines() {
        result += jolt(line, 2);
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    for line in input.lines() {
        result += jolt(line, 12);
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let problems: Vec<Vec<&str>> = input
        .lines()
        .map(|li| li.split_whitespace().collect())
        .collect();
    let rows = problems.len();
    let cols = problems[0].len();
    let mut result: u64 = 0;
    for (i, &operation) in problems[rows - 1].iter().enumerate().take(cols) {
        let mut add_result: u64 = 0;
        let mut mul_result: u64 = 1;
        let is_multiply = match operation {
            "+" => false,
            "*" => true,
            _ => panic!("Unknown operation {}", operation),
        };

        for row_vec in problems.iter().take(rows - 1) {
            let val = row_vec[i].parse::<u64>().unwrap();
            if is_multiply {
                mul_result *= val;
            } else {
                add_result += val;
            }
        }

        if is_multiply {
            result += mul_result + add_result;
        } else {
            result += add_result;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Some(0);
    }
    let rows = lines.len();
    let cols = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut grid: Vec<Vec<char>> = Vec::with_capacity(rows);
    for li in &lines {
        let mut row: Vec<char> = li.chars().collect();
        if row.len() < cols {
            row.resize(cols, ' ');
        }
        grid.push(row);
    }

    let mut grand_total: u64 = 0;

    let mut current_numbers: Vec<u64> = Vec::new();
    let mut current_op: Option<char> = None; // '+' or '*'
    let mut in_group = false;

    let is_separator = |c: usize| -> bool {
        for row in grid.iter() {
            if row[c] != ' ' {
                return false;
            }
        }
        true
    };

    let finalize_group =
        |nums: &mut Vec<u64>, op: &mut Option<char>, in_grp: &mut bool, total: &mut u64| {
            if *in_grp {
                if !nums.is_empty() {
                    let val: u64 = match op {
                        Some('+') => nums.iter().sum(),
                        Some('*') => nums.iter().copied().product(),
                        _ => nums.iter().sum(),
                    };
                    *total += val;
                }
                nums.clear();
                *op = None;
                *in_grp = false;
            }
        };

    for c in (0..cols).rev() {
        if is_separator(c) {
            finalize_group(
                &mut current_numbers,
                &mut current_op,
                &mut in_group,
                &mut grand_total,
            );
            continue;
        }

        in_group = true;

        // Determine operator from bottom row if present
        let opch = grid[rows - 1][c];
        if opch == '+' || opch == '*' && current_op.is_none() {
            current_op = Some(opch);
        }

        // Build the number from digits in this column (rows 0..rows-1)
        let mut digits: Vec<char> = Vec::new();
        for row in grid.iter().take(rows - 1) {
            let ch = row[c];
            if ch.is_ascii_digit() {
                digits.push(ch);
            }
        }
        if !digits.is_empty() {
            let num_str: String = digits.into_iter().collect();
            if let Ok(n) = num_str.parse::<u64>() {
                current_numbers.push(n);
            }
        }
    }
    // finalize last group if any
    if in_group {
        let mut tmp_in = in_group;
        finalize_group(
            &mut current_numbers,
            &mut current_op,
            &mut tmp_in,
            &mut grand_total,
        );
    }

    Some(grand_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}

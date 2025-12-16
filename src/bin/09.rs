advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let coords = input
        .lines()
        .map(|s| {
            let (x, y) = s
                .split_once(',')
                .map_or((0, 0), |(x, y)| (x.parse().unwrap(), y.parse().unwrap()));
            (x, y)
        })
        .collect::<Vec<(u64, u64)>>();
    let mut max_area: u64 = 0;
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let width = coords[i].0.abs_diff(coords[j].0);
            let height = coords[i].1.abs_diff(coords[j].1);
            if width == 0 || height == 0 {
                let mut area: u64 = 0;
                if width == 0 {
                    area = height + 1;
                }
                if height == 0 {
                    area = width + 1;
                }
                if area > max_area {
                    max_area = area;
                }
                continue;
            }
            let area = (width + 1) * (height + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    Some(max_area)
}

pub fn part_two(_: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

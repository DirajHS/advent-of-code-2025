advent_of_code::solution!(7);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, fill: T) -> Self
    where
        T: Clone,
    {
        Self {
            width,
            height,
            data: vec![fill; width * height],
        }
    }

    pub fn from_vec(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(data.len(), width * height);
        Self {
            width,
            height,
            data,
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    fn idx(&self, r: usize, c: usize) -> usize {
        r * self.width + c
    }

    #[inline]
    pub fn in_bounds(&self, r: isize, c: isize) -> bool {
        r >= 0 && c >= 0 && (r as usize) < self.height && (c as usize) < self.width
    }

    #[inline]
    pub fn get(&self, r: usize, c: usize) -> Option<&T> {
        if r < self.height && c < self.width {
            Some(&self.data[self.idx(r, c)])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut T> {
        if r < self.height && c < self.width {
            let idx = self.idx(r, c);
            Some(&mut self.data[idx])
        } else {
            None
        }
    }

    /// Iterate over all coordinates row-major: (row, col)
    pub fn coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height).flat_map(move |r| (0..self.width).map(move |c| (r, c)))
    }

    pub fn coords_rev(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + '_ {
        (0..self.height - 1)
            .rev()
            .flat_map(move |r| (0..self.width - 1).rev().map(move |c| (r, c)))
    }

    /// 8-direction neighbor coordinates around (r, c)
    pub fn neighbors8(&self, r: usize, c: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        const OFFSETS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let r = r as isize;
        let c = c as isize;
        OFFSETS
            .into_iter()
            .map(move |(dr, dc)| (r + dr, c + dc))
            .filter(move |&(nr, nc)| self.in_bounds(nr, nc))
            .map(|(nr, nc)| (nr as usize, nc as usize))
    }
}

impl Grid<char> {
    /// Construct a `Grid<char>` from a multi-line string
    pub fn from_str_chars(s: &str) -> Self {
        let lines: Vec<&str> = s.lines().collect();
        let height = lines.len();
        let width = lines.first().map(|l| l.len()).unwrap_or(0);

        let mut data = Vec::with_capacity(width * height);
        for line in lines {
            data.extend(line.chars());
        }
        Grid::from_vec(width, height, data)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = Grid::from_str_chars(input);
    let mut result: u64 = 0;
    for (r, c) in &mut grid.clone().coords() {
        match grid.get(r, c) {
            None => {}
            Some(location) => match location {
                'S' => {
                    if grid.get(r.wrapping_add(1), c.wrapping_add(0)).is_some() {
                        *grid.get_mut(r.wrapping_add(1), c.wrapping_add(0)).unwrap() = '|';
                    }
                }
                '.' => {
                    if let Some(previous_location) = grid.get(r.wrapping_sub(1), c) {
                        match previous_location {
                            'S' => {
                                *grid.get_mut(r, c).unwrap() = '|';
                            }
                            '|' => {
                                *grid.get_mut(r, c).unwrap() = '|';
                            }
                            &_ => {}
                        }
                    }
                }
                '^' => {
                    if let Some(previous_location) = grid.get(r.wrapping_sub(1), c)
                        && previous_location == &'|'
                    {
                        *grid.get_mut(r, c - 1).unwrap() = '|';
                        *grid.get_mut(r, c + 1).unwrap() = '|';
                        result += 1;
                    }
                }
                &_ => {}
            },
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::from_str_chars(input);

    // Locate start 'S'
    let (sr, sc) = match grid
        .coords()
        .find(|&(r, c)| matches!(grid.get(r, c), Some('S')))
    {
        Some(pos) => pos,
        None => return Some(0),
    };

    // widths/heights are available via the grid; no local copies needed

    // Memoization table for in-bounds cells only
    let mut memo: Vec<Option<u64>> = vec![None; (grid.width * grid.height) as usize];

    fn idx(width: usize, r: usize, c: usize) -> usize {
        r * width + c
    }

    fn count(grid: &Grid<char>, memo: &mut Vec<Option<u64>>, r: isize, c: isize) -> u64 {
        let width = grid.width as isize;
        let height = grid.height as isize;

        // Exiting the manifold counts as one completed timeline
        if r >= height {
            return 1;
        }
        if c < 0 || c >= width {
            return 1;
        }

        let ru = r as usize;
        let cu = c as usize;

        let midx = idx(grid.width, ru, cu);
        if let Some(v) = memo[midx] {
            return v;
        }

        let ch = grid.get(ru, cu).copied().unwrap_or('.');
        let res = match ch {
            '^' => {
                // Split: choose left or right (quantum timelines add)
                let left = count(grid, memo, r, c - 1);
                let right = count(grid, memo, r, c + 1);
                left + right
            }
            // Empty space or start: continue downward
            _ => count(grid, memo, r + 1, c),
        };

        memo[midx] = Some(res);
        res
    }

    Some(count(&grid, &mut memo, sr as isize, sc as isize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}

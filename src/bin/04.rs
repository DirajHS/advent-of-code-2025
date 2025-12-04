advent_of_code::solution!(4);

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
    let grid = Grid::from_str_chars(input);
    let mut result: u64 = 0;
    grid.coords()
        .filter(|(r, c)| grid.get(*r, *c) == Some(&'@'))
        .for_each(|(r, c)| {
            let neighbors = grid
                .neighbors8(r, c)
                .filter(|(r, c)| grid.get(*r, *c) == Some(&'@'))
                .count() as u64;
            if neighbors < 4 {
                result += 1
            }
        });
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::from_str_chars(input);
    let mut result = 0u64;

    let mut removable = Vec::<(usize, usize)>::new();
    loop {
        removable.clear();
        for (r, c) in grid.coords() {
            if grid.get(r, c) == Some(&'@') {
                let neighbors = grid
                    .neighbors8(r, c)
                    .filter(|&(nr, nc)| grid.get(nr, nc) == Some(&'@'))
                    .count();
                if neighbors < 4 {
                    result += 1;
                    removable.push((r, c));
                }
            }
        }

        if removable.is_empty() {
            break;
        }
        for (r, c) in &removable {
            *grid.get_mut(*r, *c).unwrap() = '.';
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}

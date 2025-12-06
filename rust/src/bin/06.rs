use std::ops::{Deref, Index, Range};

advent_of_code::solution!(6);

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
}

trait GridLike<'input>: Index<(usize, usize), Output = u8> {
    /// Height of the grid.
    fn height(&self) -> usize;
    /// Width of the grid.
    fn width(&self) -> usize;

    /// Return row `y`.
    fn row(&self, y: usize) -> impl Iterator<Item = u8> {
        (0..self.width()).map(move |x| self[(x, y)])
    }
    /// Return column `x`.
    fn col(&self, x: usize) -> impl Iterator<Item = u8> {
        (0..self.height()).map(move |y| self[(x, y)])
    }

    /// Return an iterator of all rows.
    fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = u8>> {
        (0..self.height()).map(|y| self.row(y))
    }
    /// Return an iterator of all columns.
    fn cols(&self) -> impl Iterator<Item = impl Iterator<Item = u8>> {
        (0..self.width()).map(|x| self.col(x))
    }

    /// Produce a cursor positioned at `(x, y)`.
    fn cursor(&self, x: usize, y: usize) -> Cursor<'_, Self>
    where
        Self: Sized,
    {
        Cursor { grid: self, x, y }
    }
    /// Produce a view from `(min_x, min_y)` (inclusive) to `(max_x, max_y)` (exclusive).
    fn view(&self, (min_x, min_y): (usize, usize), (max_x, max_y): (usize, usize)) -> View<'_, Self>
    where
        Self: Sized,
    {
        View {
            grid: self,
            x: min_x..max_x,
            y: min_y..max_y,
        }
    }
}

struct Grid<'input> {
    /// Underlying buffer.
    buf: &'input [u8],
    /// Width of each row (including the terminating newline).
    width: usize,
    /// Number of rows.
    height: usize,
}
impl<'input> Grid<'input> {
    pub fn new(input: &'input str) -> Self {
        let width = input.char_indices().find(|(_, c)| *c == '\n').unwrap().0 + 1;
        let height = input.len() / width;

        Self {
            buf: input.as_bytes(),
            height,
            width,
        }
    }

    fn offset(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width, "x ({x}) must be in bounds ({})", self.width);
        assert!(
            y < self.height,
            "y ({y}) must be in bounds ({})",
            self.height
        );

        (y * self.width) + x
    }
}
impl<'input> GridLike<'input> for Grid<'input> {
    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width - 1 // Account for the newline
    }
}
impl<'input> Index<(usize, usize)> for Grid<'input> {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.buf[self.offset(x, y)]
    }
}

struct Cursor<'grid, G> {
    grid: &'grid G,
    x: usize,
    y: usize,
}
impl<'grid, 'input, G: GridLike<'input>> Cursor<'grid, G> {
    pub fn in_bounds(&self) -> bool {
        self.x < self.grid.width() && self.y < self.grid.height()
    }

    pub fn right(&mut self) {
        self.x += 1;
    }
}
impl<'grid, 'input, G: GridLike<'input>> Deref for Cursor<'grid, G> {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.grid[(self.x, self.y)]
    }
}

struct View<'grid, G> {
    grid: &'grid G,
    x: Range<usize>,
    y: Range<usize>,
}
impl<'grid, 'input, G: GridLike<'input>> GridLike<'input> for View<'grid, G> {
    fn height(&self) -> usize {
        self.y.len()
    }

    fn width(&self) -> usize {
        self.x.len()
    }
}
impl<'grid, 'input, G: GridLike<'input>> Index<(usize, usize)> for View<'grid, G> {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < self.x.len());
        assert!(y < self.y.len());
        &self.grid[(self.x.start + x, self.y.start + y)]
    }
}

fn solve(input: &str, cephalopod_mode: bool) -> u64 {
    let grid = Grid::new(input);

    (0..)
        .scan(grid.cursor(0, grid.height - 1), |cursor, _| {
            if !cursor.in_bounds() {
                return None;
            }

            // Determine the column width.
            let mut width = 0;
            loop {
                cursor.right();

                if !cursor.in_bounds() {
                    width += 1;
                    break;
                }

                if !cursor.is_ascii_whitespace() {
                    break;
                }

                width += 1;
            }

            Some(width)
        })
        // Compound column widths to get the offset for each column
        .scan(0usize, |offset, width| {
            let current_offset = *offset;
            *offset += width + 1;
            Some((current_offset, width))
        })
        .map(|(offset, width)| {
            (
                // Pull out the numbers.
                grid.view((offset, 0), (offset + width, grid.height() - 1)),
                // Pull out the operation.
                grid.view((offset, grid.height() - 1), (offset + width, grid.height())),
            )
        })
        .map(move |(column, operation)| {
            let operation = match operation[(0, 0)] {
                b'+' => Operation::Add,
                b'*' => Operation::Multiply,
                c => panic!("unknown operation: {}", c as char),
            };

            fn operate(
                operation: Operation,
                values: impl Iterator<Item = impl Iterator<Item = u8>>,
            ) -> u64 {
                let values = values.map(|row| {
                    row.filter(|c| c.is_ascii_digit())
                        .map(|c| (c - b'0') as u64)
                        .reduce(|total, n| (total * 10) + n)
                        .unwrap_or(0)
                });

                match operation {
                    Operation::Add => values.sum(),
                    Operation::Multiply => values.reduce(|value, n| value * n).unwrap_or(0),
                }
            }

            if cephalopod_mode {
                operate(operation, column.cols())
            } else {
                operate(operation, column.rows())
            }
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, true))
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

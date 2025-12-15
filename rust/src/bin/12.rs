use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Index, RangeInclusive},
};

advent_of_code::solution!(12);

const IDENTITY: [[(usize, usize); 3]; 3] = [
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
];

#[derive(Clone, Debug)]
struct Present([[bool; 3]; 3]);
impl Present {
    pub fn new(shape: [[bool; 3]; 3]) -> Self {
        Self(shape)
    }

    fn size(&self) -> usize {
        IDENTITY.iter().flatten().filter(|p| self[**p]).count()
    }

    fn flip_x(&self) -> Self {
        Self::new([self.0[2], self.0[1], self.0[0]])
    }

    fn flip_y(&self) -> Self {
        Self::new(self.0.map(|row| [row[2], row[1], row[0]]))
    }

    fn rotate_90(&self) -> Self {
        Self::new([
            [self.0[2][0], self.0[1][0], self.0[0][0]],
            [self.0[2][1], self.0[1][1], self.0[0][1]],
            [self.0[2][2], self.0[1][2], self.0[0][2]],
        ])
    }

    pub fn variants(&self) -> impl Iterator<Item = Self> {
        [
            |present: &Self| present.clone(),
            Self::flip_x,
            Self::flip_y,
            Self::rotate_90,
            |present: &Self| present.rotate_90().rotate_90(),
            |present: &Self| present.rotate_90().rotate_90().rotate_90(),
            |present: &Self| present.flip_x().rotate_90(),
            |present: &Self| present.flip_y().rotate_90(),
        ]
        .into_iter()
        .map(|f| f(self))
    }
}
impl Index<(usize, usize)> for Present {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.0[y][x]
    }
}
impl Display for Present {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..3 {
            for x in 0..3 {
                write!(f, "{}", if self[(x, y)] { "#" } else { " " })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Bounds {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}
impl Bounds {
    fn merge(&mut self, (x, y): (usize, usize)) {
        self.min_x = usize::min(self.min_x, x);
        self.min_y = usize::min(self.min_y, y);
        self.max_x = usize::max(self.max_x, x);
        self.max_y = usize::max(self.max_y, y);
    }

    fn calculate_bounds(iter: impl Iterator<Item = (usize, usize)>) -> Self {
        iter.fold(Self::default(), |mut bounds, (x, y)| {
            bounds.merge((x, y));

            bounds
        })
    }

    fn dimensions(&self) -> (usize, usize) {
        (
            if self.min_x <= self.max_x {
                self.max_x - self.min_x + 1
            } else {
                0
            },
            if self.min_y <= self.max_y {
                self.max_y - self.min_y + 1
            } else {
                0
            },
        )
    }

    fn area(&self) -> usize {
        let (width, length) = self.dimensions();
        width * length
    }

    fn x(&self) -> RangeInclusive<usize> {
        self.min_x..=self.max_x
    }

    fn y(&self) -> RangeInclusive<usize> {
        self.min_y..=self.max_y
    }
}
impl Default for Bounds {
    fn default() -> Self {
        Self {
            min_x: usize::MAX,
            min_y: usize::MAX,
            max_x: usize::MIN,
            max_y: usize::MIN,
        }
    }
}

struct Grid {
    grid: [[bool; 128]; 128],
    width: usize,
    length: usize,
}
impl Grid {
    fn new(width: usize, length: usize) -> Self {
        Self {
            grid: [[false; 128]; 128],
            width,
            length,
        }
    }

    fn try_place(&mut self, (x, y): (usize, usize), present: &Present) -> bool {
        if !self.valid_place((x, y), present) {
            return false;
        }

        IDENTITY
            .iter()
            .flatten()
            .filter(|&&(x, y)| present[(x, y)])
            .for_each(|&(dx, dy)| self.grid[y + dy][x + dx] = true);

        true
    }

    fn valid_place(&self, (x, y): (usize, usize), present: &Present) -> bool {
        self.overlay((x, y), present)
            .iter()
            .flatten()
            .all(|overlay| !*overlay)
            && {
                let bounds = self.overlay_bounds((x, y), present);
                let (width, length) = bounds.dimensions();
                width <= self.width && length <= self.length
            }
    }

    /// Bounds of the present was overlaid at the provided position. Does not verify that the
    /// position is valid.
    fn overlay_bounds(&self, (x, y): (usize, usize), present: &Present) -> Bounds {
        let mut bounds = self.bounds();
        IDENTITY
            .into_iter()
            .flatten()
            .filter(|&(x, y)| present[(x, y)])
            .map(|(dx, dy)| (x + dx, y + dy))
            .for_each(|(x, y)| bounds.merge((x, y)));
        bounds
    }

    fn overlay(&self, (x, y): (usize, usize), present: &Present) -> [[bool; 3]; 3] {
        IDENTITY.map(|row| {
            row.map(|(offset_x, offset_y)| {
                present[(offset_x, offset_y)] & self.grid[y + offset_y][x + offset_x]
            })
        })
    }

    fn bounds(&self) -> Bounds {
        Bounds::calculate_bounds(
            (0..self.grid.len())
                .flat_map(|y| (0..self.grid[0].len()).map(move |x| (x, y)))
                .filter(|(x, y)| self.grid[*y][*x]),
        )
    }

    /// Produce an iterator of all available insertion points.
    fn insertion_points(&self) -> Vec<(usize, usize)> {
        let bounds = self.bounds();

        // Find all empty squares within the current bounds.
        bounds
            .y()
            .flat_map({
                let bounds = bounds.clone();
                move |y| bounds.x().map(move |x| (x, y))
            })
            .filter(|&(x, y)| !self.grid[y][x])
            .flat_map(|(x, y)| {
                // HACK: Account for holes in the corner.
                [-1, 1]
                    .iter()
                    .flat_map(|d| {
                        IDENTITY
                            .iter()
                            .flatten()
                            .map(move |&(x, y)| (x as isize * d, y as isize * d))
                    })
                    .map(move |(dx, dy)| {
                        (
                            x.checked_add_signed(dx).unwrap(),
                            y.checked_add_signed(dy).unwrap(),
                        )
                    })
            })
            .chain(
                // Default point in-case the grid is empty.
                [(self.grid.len() / 2, self.grid[0].len() / 2)],
            )
            .chain(
                // Add in points above/below y bounds.
                bounds
                    .x()
                    .flat_map(move |x| [(x, bounds.max_y + 1), (x, bounds.min_y - 3)]),
            )
            .chain(
                // Add in points above/below x bounds.
                bounds
                    .y()
                    .flat_map(move |y| [(bounds.max_x + 1, y), (bounds.min_x - 3, y)]),
            )
            // Deduplicate.
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.bounds().y() {
            for x in self.bounds().x() {
                write!(f, "{}", if self.grid[y][x] { "#" } else { " " })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn can_fit((width, length): (usize, usize), presents: &[Present], mut amounts: Vec<usize>) -> bool {
    let required_space = presents
        .iter()
        .zip(amounts.iter())
        .map(|(present, amount)| present.size() * amount)
        .sum::<usize>();
    let actual_space = width * length;
    if required_space > actual_space {
        return false;
    } else {
        return true;
    }

    // Pre-compute all the available presents.
    let presents = presents
        .iter()
        .map(|present| present.variants().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut grid = Grid::new(width, length);

    while amounts.iter().sum::<usize>() > 0 {
        let points = grid.insertion_points();

        let bounds = grid.bounds();
        let (width, length) = bounds.dimensions();

        let Some((i, present, point)) = presents
            .iter()
            .enumerate()
            .filter(|(i, _)| amounts[*i] > 0)
            .flat_map(|(i, presents)| presents.iter().map(move |present| (i, present)))
            .flat_map(|(i, present)| points.iter().map(move |point| (i, present, *point)))
            .filter(|(_, present, point)| grid.valid_place(*point, present))
            .min_by_key(|(_, present, point)| {
                let bounds = grid.overlay_bounds(*point, present);
                let (new_width, new_length) = bounds.dimensions();

                (
                    new_width - width + new_length - length,
                    bounds.area(),
                    present.size(),
                )
            })
        else {
            return false;
        };

        assert!(grid.try_place(point, present));
        amounts[i] -= 1;

        println!("placing at {point:?}:");
        println!(
            "delta dimensions: {:?}",
            (
                grid.bounds().dimensions().0 - width,
                grid.bounds().dimensions().1 - length
            )
        );
        println!("{present}");

        println!("grid:\n{grid}");
    }

    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let (presents, regions) = input
        .split("\n\n")
        .map(|section| section.lines().peekable())
        .fold(
            (Vec::new(), None),
            |(mut presents, mut regions), mut section| {
                if section.peek().unwrap().ends_with(':') {
                    section.next().unwrap();

                    presents.push(Present::new(
                        TryInto::<[[bool; 3]; 3]>::try_into(
                            section
                                .map(|line| -> [bool; 3] {
                                    TryInto::<[bool; 3]>::try_into(
                                        line.as_bytes()
                                            .iter()
                                            .map(|&b| b == b'#')
                                            .collect::<Vec<_>>(),
                                    )
                                    .unwrap()
                                })
                                .collect::<Vec<_>>(),
                        )
                        .unwrap(),
                    ));
                } else {
                    regions = Some(
                        section
                            .map(|line| line.split_once(": ").unwrap())
                            .map(|(range, presents)| {
                                (
                                    {
                                        let (width, length) = range.split_once('x').unwrap();
                                        (width.parse().unwrap(), length.parse().unwrap())
                                    },
                                    presents
                                        .split(' ')
                                        .map(|n| n.parse().unwrap())
                                        .collect::<Vec<_>>(),
                                )
                            })
                            .collect::<Vec<_>>(),
                    );
                }

                (presents, regions)
            },
        );
    let regions = regions.unwrap();

    Some(
        regions
            .into_iter()
            .filter(|(region, amounts)| can_fit(*region, &presents, amounts.clone()))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

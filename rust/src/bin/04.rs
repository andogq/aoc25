use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

advent_of_code::solution!(4);

fn deltas() -> impl Iterator<Item = (isize, isize)> {
    (-1..=1)
        .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
        .filter(|&(dx, dy)| (dx, dy) != (0, 0))
}

#[derive(Clone)]
struct Map {
    buf: Box<[bool]>,
    height: usize,
    width: usize,
}
impl Map {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            buf: vec![false; width * height].into_boxed_slice(),
            height,
            width,
        }
    }

    pub fn parse(input: &str) -> Self {
        let width = input.char_indices().find(|(_, c)| *c == '\n').unwrap().0;
        let height = input.len() / (width + 1);

        let mut map = Self::new(height, width);

        input
            .chars()
            .filter(|&c| c != '\n')
            .enumerate()
            .for_each(|(i, c)| map.buf[i] = c == '@');

        map
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.buf.get(self.offset(x, y)?).cloned()
    }

    pub fn accessible(&self, threshold: usize) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .filter_map(|(pos, paper)| paper.then_some(pos))
            .filter(move |(x, y)| {
                deltas()
                    .flat_map(|(dx, dy)| {
                        Some((x.checked_add_signed(dx)?, y.checked_add_signed(dy)?))
                    })
                    .flat_map(|(x, y)| self.get(x, y))
                    .filter(|paper| *paper)
                    .count()
                    < threshold
            })
    }

    fn offset(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some((y * self.width) + x)
    }

    fn iter(&self) -> impl Iterator<Item = ((usize, usize), bool)> {
        self.buf
            .iter()
            .enumerate()
            .map(|(i, b)| ((i % self.width, i / self.width), *b))
    }
}
impl Index<(usize, usize)> for Map {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.buf[self.offset(x, y).unwrap()]
    }
}
impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.buf[self.offset(x, y).unwrap()]
    }
}
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.buf.iter().enumerate().try_for_each(|(i, c)| {
            write!(f, "{}", if *c { "@" } else { "." })?;

            if (i + 1) % (self.width) == 0 {
                writeln!(f)?;
            }

            Ok(())
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(Map::parse(input).accessible(4).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        (0..)
            .scan(Map::parse(input), |state, _| {
                Some(
                    state
                        .clone()
                        .accessible(4)
                        .inspect(|&(x, y)| {
                            state[(x, y)] = false;
                        })
                        .count() as u64,
                )
            })
            .take_while(|&count| count != 0)
            .sum(),
    )
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

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut accessible = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if !map[y][x] {
                continue;
            }

            let surrounding = (-1..=1)
                .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                .filter(|&(dx, dy)| (dx, dy) != (0, 0))
                .flat_map(|(dx, dy)| Some((x.checked_add_signed(dx)?, y.checked_add_signed(dy)?)))
                .flat_map(|(x, y)| Some(((x, y), *map.get(y)?.get(x)?)))
                .filter(|(_, paper)| *paper)
                .count();

            if surrounding < 4 {
                accessible += 1;
            }
        }
    }

    Some(accessible)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut accessible = 0;

    loop {
        let mut delta = 0;
        let mut next_map = map.clone();

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if !map[y][x] {
                    continue;
                }

                let surrounding = (-1..=1)
                    .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                    .filter(|&(dx, dy)| (dx, dy) != (0, 0))
                    .flat_map(|(dx, dy)| {
                        Some((x.checked_add_signed(dx)?, y.checked_add_signed(dy)?))
                    })
                    .flat_map(|(x, y)| Some(((x, y), *map.get(y)?.get(x)?)))
                    .filter(|(_, paper)| *paper)
                    .count();

                if surrounding < 4 {
                    delta += 1;
                    next_map[y][x] = false;
                }
            }
        }

        if delta == 0 {
            break;
        }

        accessible += delta;
        map = next_map;
    }

    Some(accessible)
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

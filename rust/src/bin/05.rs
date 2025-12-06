use std::ops::RangeInclusive;

advent_of_code::solution!(5);

fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (ranges, available) = input.split_once("\n\n").unwrap();

    (
        ranges
            .lines()
            .map(|line| line.split_once("-").unwrap())
            .map(|(lhs, rhs)| lhs.parse().unwrap()..=rhs.parse().unwrap())
            .collect(),
        available.lines().map(|n| n.parse().unwrap()).collect(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, available) = parse(input);

    Some(
        available
            .iter()
            .filter(|n| ranges.iter().any(|range| range.contains(n)))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut ranges, _) = parse(input);

    loop {
        let mut modified = false;

        ranges = ranges
            .iter()
            .cloned()
            .enumerate()
            .flat_map(|(i, mut range)| {
                for prev in ranges.iter().take(i) {
                    match (prev.contains(range.start()), prev.contains(range.end())) {
                        (true, true) => {
                            modified = true;
                            return [None, None];
                        }
                        (true, false) => {
                            range = *prev.end() + 1..=*range.end();
                            modified = true;
                        }
                        (false, true) => {
                            range = *range.start()..=*prev.start() - 1;
                            modified = true;
                        }
                        (false, false)
                            if range.start() < prev.start() && range.end() > prev.end() =>
                        {
                            modified = true;
                            return [
                                Some(*range.start()..=*prev.start() - 1),
                                Some(*prev.end() + 1..=*range.end()),
                            ];
                        }
                        (false, false) => continue,
                    }
                }

                assert!(range.start() <= range.end());

                [Some(range), None]
            })
            .flatten()
            .collect();

        if !modified {
            break;
        }
    }

    Some(
        ranges
            .into_iter()
            .inspect(|r| {
                dbg!(r);
            })
            .flatten()
            .count() as u64,
    )
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
        assert_eq!(result, Some(14));
    }
}

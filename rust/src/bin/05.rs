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

    let mut solutions = Vec::<RangeInclusive<u64>>::with_capacity(ranges.len());

    let mut i = 0;
    'outer: while i < ranges.len() {
        let range = ranges[i].clone();

        for prev in &solutions {
            match (prev.contains(range.start()), prev.contains(range.end())) {
                (true, true) => {
                    // This range is already fulfilled by a solution, so skip it.
                    i += 1;
                    continue 'outer;
                }
                (true, false) => {
                    // Update this range to not overlap an existing solution, then retry.
                    ranges[i] = *prev.end() + 1..=*range.end();
                    continue 'outer;
                }
                (false, true) => {
                    // Update this range to not overlap an existing solution, then retry.
                    ranges[i] = *range.start()..=*prev.start() - 1;
                    continue 'outer;
                }
                (false, false) if range.start() < prev.start() && range.end() > prev.end() => {
                    // Split this range into two, to straddle an existing solution. Use one to
                    // replace the current range, and append the other to be searched later.
                    ranges[i] = *range.start()..=*prev.start() - 1;
                    ranges.push(*prev.end() + 1..=*range.end());
                    continue 'outer;
                }
                // No overlap against solution range, continue searching.
                (false, false) => continue,
            }
        }

        // If reached here, this range has no overlap with an existing solution. Save it, then
        // continue onto the next range.
        solutions.push(range);
        i += 1;
    }

    Some(solutions.into_iter().flatten().count() as u64)
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

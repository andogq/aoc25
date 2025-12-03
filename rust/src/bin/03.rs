advent_of_code::solution!(3);

fn largest(batteries: &[u64], remaining: u32) -> u64 {
    if batteries.len() < remaining as usize || remaining == 0 {
        return 0;
    }

    let max = *batteries
        .iter()
        .take(batteries.len() - remaining as usize + 1)
        .max()
        .unwrap();
    let i = batteries
        .iter()
        .enumerate()
        .find(|(_, b)| **b == max)
        .unwrap()
        .0;

    u64::max(
        max * 10u64.pow(remaining - 1) + largest(&batteries[i + 1..], remaining - 1),
        largest(&batteries[i + 1..], remaining),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|bank| {
                bank.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect::<Vec<_>>()
            })
            .map(|bank| largest(&bank, 2))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|bank| {
                bank.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect::<Vec<_>>()
            })
            .map(|bank| largest(&bank, 12))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}

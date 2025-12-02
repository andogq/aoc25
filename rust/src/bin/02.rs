advent_of_code::solution!(2);

fn is_repeated(s: &str) -> bool {
    s.len().is_multiple_of(2) && s[0..s.len() / 2] == s[s.len() / 2..]
}

fn is_repeated_electric_boogaloo(s: &str) -> bool {
    (1..=s.len() / 2)
        .filter(|i| s.len().is_multiple_of(*i))
        .any(|i| {
            let mut ranges = (0..(s.len() / i)).map(|start| start * i..(start + 1) * i);

            let first = &s[ranges.next().unwrap()];

            ranges.all(|range| first == &s[range])
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .split(",")
            .map(|range| range.split_once("-").unwrap())
            .flat_map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap())
            .filter(|id| is_repeated(&id.to_string()))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .split(",")
            .map(|range| range.split_once("-").unwrap())
            .flat_map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap())
            .filter(|id| is_repeated_electric_boogaloo(&id.to_string()))
            .sum(),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn asdfasdf() {
        assert!(!is_repeated_electric_boogaloo("2121212118"));
    }
    #[test]
    fn asdfasdf2() {
        assert!(is_repeated_electric_boogaloo("2121212121"));
    }
}

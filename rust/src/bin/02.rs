advent_of_code::solution!(2);

fn is_repeated(n: u64) -> bool {
    repeated_times(n, 2)
}

fn repeated_times(n: u64, i: u32) -> bool {
    let tens = n.ilog10() + 1;

    if !tens.is_multiple_of(i) {
        return false;
    }

    let digits = tens / i;

    let cmp = extract_digits(n, 0, digits);

    (1..i).all(|j| cmp == extract_digits(n, j * digits, digits))
}

fn extract_digits(n: u64, start: u32, count: u32) -> u64 {
    n / 10u64.pow(start) % 10u64.pow(count)
}

fn is_repeated_electric_boogaloo(n: u64) -> bool {
    let tens = n.ilog10() + 1;

    (2..=tens).any(|i| repeated_times(n, i))
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .split(",")
            .map(|range| range.split_once("-").unwrap())
            .flat_map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap())
            .filter(|id| is_repeated(*id))
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
            .filter(|id| is_repeated_electric_boogaloo(*id))
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
        assert!(!is_repeated(1000));
    }
    #[test]
    fn asdfasdf1() {
        assert!(is_repeated(22));
    }
    #[test]
    fn asdfasdf11() {
        assert_eq!(extract_digits(22, 1, 1), 2);
    }
    #[test]
    fn asdfasdf2() {
        assert!(is_repeated_electric_boogaloo(2121212121));
    }

    #[test]
    fn digits1() {
        assert_eq!(extract_digits(123, 0, 1), 3);
    }
    #[test]
    fn digits2() {
        assert_eq!(extract_digits(123, 1, 1), 2);
    }
    #[test]
    fn digits3() {
        assert_eq!(extract_digits(123, 2, 1), 1);
    }
    #[test]
    fn digits4() {
        assert_eq!(extract_digits(123, 1, 2), 12);
    }
}

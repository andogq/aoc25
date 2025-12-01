advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let dir = line.chars().next().unwrap();
                let num = line[1..].parse::<i64>().unwrap();

                num * match dir {
                    'L' => -1,
                    'R' => 1,
                    dir => panic!("unknown dir {dir}"),
                }
            })
            .fold((50, 0), |(mut accum, mut count), dir| {
                accum = (accum + (dir + 100)) % 100;

                if accum == 0 {
                    count += 1;
                }

                (accum, count)
            })
            .1,
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

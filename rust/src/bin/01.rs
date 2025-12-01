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
            .fold((50i64, 0), |(mut accum, mut count), dir| {
                // Count full rotations.
                count += dir.unsigned_abs() / 100;

                let end = accum + (dir % 100);

                if accum != 0 && (end <= 0 || end > 99) {
                    count += 1;
                }

                accum = (end + 100) % 100;

                (accum, count)
            })
            .1,
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn extra() {
        // assert_eq!(part_two("R1000"), Some(10));
        assert_eq!(part_two("L50\nR100"), Some(2));
    }

    #[test]
    fn mini() {
        assert_eq!(part_two("L68"), Some(1));
    }
}

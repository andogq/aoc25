advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().map(|line| line.as_bytes());

    let width = lines.next().unwrap().len();
    let mut beams = vec![false; width];
    beams[width / 2] = true;

    Some(
        lines
            .fold((beams, 0), |(beams, mut count), line| {
                let mut next_beams = beams.clone();

                beams
                    .into_iter()
                    .enumerate()
                    .filter(|(_, b)| *b)
                    .filter(|(i, _)| line[*i] == b'^')
                    .inspect(|_| {
                        count += 1;
                    })
                    .flat_map(|(i, _)| [(i + 1, true), (i, false), (i - 1, true)])
                    .for_each(|(i, v)| {
                        next_beams[i] = v;
                    });

                (next_beams, count)
            })
            .1,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().map(|line| line.as_bytes());

    let width = lines.next().unwrap().len();
    let mut beams = vec![0; width];
    beams[width / 2] = 1;

    Some(
        lines
            .fold(beams, |beams, line| {
                let mut next_beams = beams.clone();

                beams
                    .into_iter()
                    .enumerate()
                    .filter(|(_, b)| *b > 0)
                    .filter(|(i, _)| line[*i] == b'^')
                    .flat_map(|(i, b)| [(i + 1, Some(b)), (i, None), (i - 1, Some(b))])
                    .for_each(|(i, delta)| {
                        if let Some(delta) = delta {
                            next_beams[i] += delta;
                        } else {
                            next_beams[i] = 0
                        }
                    });

                next_beams
            })
            .into_iter()
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let boxes = input
        .lines()
        .map(|line| line.split(','))
        .map(|mut iter| {
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    // Changes between example and input.
    let amount = if boxes.len() == 20 { 10 } else { 1000 };

    let mut circuits: Vec<_> = (0..boxes.len()).map(|i| vec![i]).collect();

    {
        let mut distances = (0..boxes.len())
            .flat_map(|a| (a + 1..boxes.len()).map(move |b| (a, b)))
            .map(|(a, b)| {
                ((a, b), {
                    let (ax, ay, az) = &boxes[a];
                    let (bx, by, bz) = &boxes[b];

                    (usize::pow(ax.abs_diff(*bx), 2)
                        + usize::pow(ay.abs_diff(*by), 2)
                        + usize::pow(az.abs_diff(*bz), 2))
                    .isqrt()
                })
            })
            .collect::<Vec<_>>();
        distances.sort_unstable_by_key(|(_, distance)| *distance);
        distances.into_iter().take(amount)
    }
    .for_each(|((a, b), _)| {
        let a_circuit = circuits
            .iter()
            .enumerate()
            .find(|(_, boxes)| boxes.contains(&a))
            .map(|(i, _)| i);
        let b_circuit = circuits
            .iter()
            .enumerate()
            .find(|(_, boxes)| boxes.contains(&b))
            .map(|(i, _)| i);

        match (a_circuit, b_circuit) {
            (Some(a), Some(b)) if a == b => {}
            (Some(a), Some(b)) => {
                let circuit = std::mem::take(&mut circuits[b]);
                circuits[a].extend(circuit);
                circuits.remove(b);
            }
            (Some(a_circuit), None) => {
                circuits[a_circuit].push(b);
            }
            (None, Some(b_circuit)) => {
                circuits[b_circuit].push(a);
            }
            (None, None) => unreachable!(),
        }
    });

    let mut lens = circuits.iter().map(|c| c.len()).collect::<Vec<_>>();
    lens.sort_unstable();

    lens.into_iter()
        .rev()
        .take(3)
        .reduce(|tot, a| tot * a)
        .map(|a| a as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes = input
        .lines()
        .map(|line| line.split(','))
        .map(|mut iter| {
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut circuits: Vec<_> = (0..boxes.len()).map(|i| vec![i]).collect();

    let distances = {
        let mut distances = (0..boxes.len())
            .flat_map(|a| (a + 1..boxes.len()).map(move |b| (a, b)))
            .map(|(a, b)| {
                ((a, b), {
                    let (ax, ay, az) = &boxes[a];
                    let (bx, by, bz) = &boxes[b];

                    (usize::pow(ax.abs_diff(*bx), 2)
                        + usize::pow(ay.abs_diff(*by), 2)
                        + usize::pow(az.abs_diff(*bz), 2))
                    .isqrt()
                })
            })
            .collect::<Vec<_>>();
        distances.sort_unstable_by_key(|(_, distance)| *distance);
        distances.into_iter()
    };

    for ((a, b), _) in distances {
        let a_circuit = circuits
            .iter()
            .enumerate()
            .find(|(_, boxes)| boxes.contains(&a))
            .map(|(i, _)| i);
        let b_circuit = circuits
            .iter()
            .enumerate()
            .find(|(_, boxes)| boxes.contains(&b))
            .map(|(i, _)| i);

        match (a_circuit, b_circuit) {
            (Some(a), Some(b)) if a == b => {}
            (Some(a), Some(b)) => {
                let circuit = std::mem::take(&mut circuits[b]);
                circuits[a].extend(circuit);
                circuits.remove(b);
            }
            (Some(a_circuit), None) => {
                circuits[a_circuit].push(b);
            }
            (None, Some(b_circuit)) => {
                circuits[b_circuit].push(a);
            }
            (None, None) => unreachable!(),
        }

        if circuits.len() == 1 {
            return Some((boxes[a].0 * boxes[b].0) as u64);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}

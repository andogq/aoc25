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

    let mut distances = vec![vec![None; boxes.len()]; boxes.len()];

    for a in 0..boxes.len() {
        for b in 0..boxes.len() {
            if a == b {
                continue;
            }

            let (ax, ay, az) = &boxes[a];
            let (bx, by, bz) = &boxes[b];

            let distance = (usize::pow(ax.abs_diff(*bx), 2)
                + usize::pow(ay.abs_diff(*by), 2)
                + usize::pow(az.abs_diff(*bz), 2))
            .isqrt();

            distances[a][b] = Some(distance);
            distances[b][a] = Some(distance);
        }
    }

    let mut circuits: Vec<Vec<usize>> = vec![];

    // Changes between example and input.
    let amount = if distances.len() == 20 { 10 } else { 1000 };

    for _ in 0..amount {
        let Some(((a, b), _)) = (0..boxes.len())
            .flat_map(|a| (0..boxes.len()).map(move |b| (a, b)))
            .flat_map(|(a, b)| Some(((a, b), distances[a][b]?)))
            .min_by_key(|(_, distance)| *distance)
        else {
            break;
        };

        // Ensure distances are cleared.
        distances[a][b] = None;
        distances[b][a] = None;

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
            }
            (Some(a_circuit), None) => {
                circuits[a_circuit].push(b);
            }
            (None, Some(b_circuit)) => {
                circuits[b_circuit].push(a);
            }
            (None, None) => circuits.push(vec![a, b]),
        }
    }

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

    let mut distances = vec![vec![None; boxes.len()]; boxes.len()];

    for a in 0..boxes.len() {
        for b in 0..boxes.len() {
            if a == b {
                continue;
            }

            let (ax, ay, az) = &boxes[a];
            let (bx, by, bz) = &boxes[b];

            let distance = (usize::pow(ax.abs_diff(*bx), 2)
                + usize::pow(ay.abs_diff(*by), 2)
                + usize::pow(az.abs_diff(*bz), 2))
            .isqrt();

            distances[a][b] = Some(distance);
            distances[b][a] = Some(distance);
        }
    }

    let mut circuits: Vec<_> = (0..boxes.len()).map(|i| vec![i]).collect();

    for _ in 0.. {
        let Some(((a, b), _)) = (0..boxes.len())
            .flat_map(|a| (0..boxes.len()).map(move |b| (a, b)))
            .flat_map(|(a, b)| Some(((a, b), distances[a][b]?)))
            .min_by_key(|(_, distance)| *distance)
        else {
            break;
        };

        // Ensure distances are cleared.
        distances[a][b] = None;
        distances[b][a] = None;

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

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let pairs = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect::<Vec<_>>();

    (0..pairs.len())
        .flat_map(|a| (a + 1..pairs.len()).map(move |b| (a, b)))
        .map(|(a, b)| (&pairs[a], &pairs[b]))
        .map(|(a, b)| (u64::abs_diff(a.0, b.0) + 1) * (u64::abs_diff(a.1, b.1) + 1))
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let pairs = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(l, r)| (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()))
        .collect::<Vec<_>>();

    let mut rects = (0..pairs.len())
        .flat_map(|a| (a + 1..pairs.len()).map(move |b| (a, b)))
        .map(|(a, b)| (&pairs[a], &pairs[b]))
        .map(|(a, b)| {
            (
                (a, b),
                (u64::abs_diff(a.0, b.0) + 1) * (u64::abs_diff(a.1, b.1) + 1),
            )
        })
        .collect::<Vec<_>>();
    rects.sort_unstable_by_key(|(_, area)| u64::MAX - *area);

    // Add midpoints.
    let pairs = pairs
        .windows(2)
        .chain([[pairs[pairs.len() - 1], pairs[0]].as_slice()])
        .flat_map(|points| {
            let a = points[0];
            let b = points[1];

            let cuts = 4;
            (0..cuts).map(move |i| {
                (
                    a.0.checked_add_signed(i * (b.0 as i64 - a.0 as i64) / cuts)
                        .unwrap(),
                    a.1.checked_add_signed(i * (b.1 as i64 - a.1 as i64) / cuts)
                        .unwrap(),
                )
            })
        })
        .collect::<Vec<_>>();

    let offset = pairs
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| {
            [
                pairs[((i + pairs.len()) - 1) % pairs.len()],
                (x, y),
                pairs[((i + pairs.len()) + 1) % pairs.len()],
            ]
        })
        .map(|pairs| pairs.map(|(x, y)| (x as isize, y as isize)))
        .map(|[a, b, c]| {
            fn vector((ax, ay): (isize, isize), (bx, by): (isize, isize)) -> (isize, isize) {
                (bx - ax, by - ay)
            }

            fn rotate((x, y): (isize, isize)) -> (isize, isize) {
                (y, -x)
            }

            fn normalise((x, y): (isize, isize)) -> (f64, f64) {
                let len = f64::sqrt(((x * x) + (y * y)) as f64);
                (x as f64 / len, y as f64 / len)
            }

            let ab = vector(a, b);
            let bc = vector(b, c);

            let v1 = rotate(ab);
            let v2 = rotate(bc);

            let v = normalise((v1.0 + v2.0, v1.1 + v2.1));
            (b.0 as f64 + (v.0 * 0.5), b.1 as f64 + (v.1 * 0.5))
        })
        .collect::<Vec<_>>();

    'outer: for &((&a, &b), area) in &rects {
        let min_x = u64::min(a.0, b.0);
        let max_x = u64::max(a.0, b.0);
        let min_y = u64::min(a.1, b.1);
        let max_y = u64::max(a.1, b.1);

        for (i, &(x, y)) in pairs.iter().enumerate() {
            if ((min_x + 1)..max_x).contains(&x) && ((min_y + 1)..max_y).contains(&y) {
                // Point within rectangle, skip it.
                continue 'outer;
            }

            // If point on edge.
            if ((x == min_x || x == max_x) && (min_y..=max_y).contains(&y))
                || ((y == min_y || y == max_y) && (min_x..=max_x).contains(&x))
            {
                // Check if the offset point is within.
                let (x, y) = offset[i];

                if x > min_x as f64 && x < max_x as f64 && y > min_y as f64 && y < max_y as f64 {
                    continue 'outer;
                }
            }
        }

        return Some(area);
    }

    None
}

// 122383625: Low
// 2440276020: High
// 123274025: Low
// 173149830

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}

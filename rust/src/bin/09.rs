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

fn ccw(a: (u64, u64), b: (u64, u64), c: (u64, u64)) -> bool {
    let ax = a.0 as i64;
    let ay = a.1 as i64;
    let bx = b.0 as i64;
    let by = b.1 as i64;
    let cx = c.0 as i64;
    let cy = c.1 as i64;

    (cy - ay) * (bx - ax) > (by - ay) * (cx - ax)
}

pub fn part_two(input: &str) -> Option<u64> {
    let pairs = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect::<Vec<_>>();

    (0..pairs.len())
        .flat_map(|a| (a + 1..pairs.len()).map(move |b| (a, b)))
        .map(|(a, b)| (&pairs[a], &pairs[b]))
        .filter(|(a, b)| {
            let w = u64::abs_diff(b.0, a.0);
            let h = u64::abs_diff(b.1, a.1);

            let x = u64::min(a.0, b.0);
            let y = u64::min(a.1, b.1);

            pairs
                .iter()
                .all(|&(xa, ya)| (xa < x || xa > (x + w)) && (ya < y || ya > (y + h)))

            // [
            //     ((x, y), (x, y + h)),
            //     ((x, y), (x + w, y)),
            //     ((x + w, y), (x + w, y + h)),
            //     ((x, y + h), (x + w, y + h)),
            // ]
            // .into_iter()
            // .all(|(start, end)| {
            //     !pairs
            //         .iter()
            //         .zip(pairs.iter().skip(1).chain([&pairs[0]]))
            //         .any(|(sub_start, sub_end)| {
            //             let intersects = [start, end].into_iter().any(|p| {
            //                 let dxc = p.0 as i64 - sub_start.0 as i64;
            //                 let dyc = p.1 as i64 - sub_start.1 as i64;
            //
            //                 let dxl = sub_end.0 as i64 - sub_start.0 as i64;
            //                 let dyl = sub_end.1 as i64 - sub_start.1 as i64;
            //
            //                 let cross = dxc * dyl - dyc * dxl;
            //                 cross == 0
            //             });
            //
            //             intersects
            //                 && (ccw(start, *sub_start, *sub_end) != ccw(end, *sub_start, *sub_end)
            //                     && ccw(start, end, *sub_start) != ccw(start, end, *sub_end))
            //         })
            // })
        })
        .map(|(a, b)| (u64::abs_diff(a.0, b.0) + 1) * (u64::abs_diff(a.1, b.1) + 1))
        .max()
}

// 122383625: Low
// 2440276020: High

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
        // assert_eq!(result, Some(24));
    }
}

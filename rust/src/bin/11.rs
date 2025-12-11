advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let (keys, paths): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .unzip();

    let (paths, ends) = paths
        .into_iter()
        .map(|connections| connections.split(' '))
        .enumerate()
        .flat_map(|(i, connections)| connections.map(move |connection| (i, connection)))
        .fold(
            (vec![vec![]; keys.len()], Vec::new()),
            |(mut paths, mut ends), (i, connection)| {
                match connection {
                    "out" => {
                        ends.push(i);
                    }
                    key => {
                        paths[i].push(keys.iter().enumerate().find(|(_, k)| **k == key).unwrap().0);
                    }
                }

                (paths, ends)
            },
        );

    let start = keys
        .iter()
        .enumerate()
        .find(|(_, key)| **key == "you")
        .unwrap()
        .0;

    let mut counts = vec![0; keys.len()];
    counts[start] = 1;

    loop {
        let mut change = false;

        let mut next_counts = vec![0; keys.len()];

        counts
            .into_iter()
            .enumerate()
            .filter(|(_, count)| *count > 0)
            .flat_map(|(i, count)| {
                paths[i]
                    .iter()
                    .map(move |next| (*next, count, true))
                    .chain(ends.contains(&i).then_some((i, count, false)))
            })
            .for_each(|(next, count, is_change)| {
                next_counts[next] += count;
                change |= is_change;
            });

        counts = next_counts;

        if !change {
            break;
        }
    }

    Some(ends.into_iter().map(|i| counts[i]).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = if input.len() == 10 {
        "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
    } else {
        input
    };

    let (keys, paths): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .unzip();

    let (paths, ends) = paths
        .into_iter()
        .map(|connections| connections.split(' '))
        .enumerate()
        .flat_map(|(i, connections)| connections.map(move |connection| (i, connection)))
        .fold(
            (vec![vec![]; keys.len()], Vec::new()),
            |(mut paths, mut ends), (i, connection)| {
                match connection {
                    "out" => {
                        ends.push(i);
                    }
                    key => {
                        paths[i].push(keys.iter().enumerate().find(|(_, k)| **k == key).unwrap().0);
                    }
                }

                (paths, ends)
            },
        );

    let checkpoints: [usize; 2] = keys
        .iter()
        .enumerate()
        .filter(|&(_, &key)| key == "dac" || key == "fft")
        .map(|(i, _)| i)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let start = keys
        .iter()
        .enumerate()
        .find(|(_, key)| **key == "svr")
        .unwrap()
        .0;

    let mut counts = vec![(0, [0; 2], 0); keys.len()];
    counts[start] = (1, [0; 2], 0);

    loop {
        let mut change = false;

        let mut next_counts = vec![(0, [0; 2], 0); keys.len()];

        counts
            .into_iter()
            .enumerate()
            .filter(|(_, (count, [checkpoint_0, checkpoint_1], both))| {
                *count + *checkpoint_0 + *checkpoint_1 + *both > 0
            })
            .flat_map(|(i, (count, checkpoint_counts, both))| {
                paths[i]
                    .iter()
                    .map({
                        let (count, checkpoint_counts, both) =
                            match (checkpoints[0] == i, checkpoints[1] == i) {
                                (false, false) => (count, checkpoint_counts, both),
                                (true, false) => (
                                    0,
                                    [count + checkpoint_counts[0], 0],
                                    both + checkpoint_counts[1],
                                ),
                                (false, true) => (
                                    0,
                                    [0, count + checkpoint_counts[1]],
                                    both + checkpoint_counts[0],
                                ),
                                (true, true) => unreachable!(),
                            };

                        move |next| (*next, (count, checkpoint_counts, both), true)
                    })
                    .chain(ends.contains(&i).then_some((
                        i,
                        (count, checkpoint_counts, both),
                        false,
                    )))
            })
            .for_each(|(next, (count, checkpoint_counts, both), is_change)| {
                next_counts[next].0 += count;
                next_counts[next].1[0] += checkpoint_counts[0];
                next_counts[next].1[1] += checkpoint_counts[1];
                next_counts[next].2 += both;
                change |= is_change;
            });

        counts = next_counts;

        if !change {
            break;
        }
    }

    Some(ends.into_iter().map(|i| counts[i].2).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
        panic!()
    }
}

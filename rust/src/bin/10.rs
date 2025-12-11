use std::collections::VecDeque;

advent_of_code::solution!(10);

// 17929: too low
// 18000: too low
// 18040: too low
// 18092: ???

fn build_usize(iter: impl Iterator<Item = bool>) -> usize {
    iter.fold(0, |value, b| (value << 1) | (b as usize) & 1)
}

fn solve(matrix: Vec<Vec<f64>>) -> u64 {
    fn really_solve(mut matrix: Vec<Vec<f64>>) -> Option<Vec<Option<f64>>> {
        let n = matrix.len();
        let m = matrix[0].len() - 1;

        let mut w = vec![None; m];

        let mut col = 0;
        let mut row = 0;
        while col < m && row < n {
            // Select the pivot as the largest absolute value in the column.
            let pivot = (row..n)
                .reduce(|pivot, i| {
                    if matrix[i][col].abs() > matrix[pivot][col].abs() {
                        i
                    } else {
                        pivot
                    }
                })
                .unwrap();

            if matrix[pivot][col].abs() < 10e-6 {
                col += 1;
                continue;
            }

            // Pivot the row.
            matrix.swap(row, pivot);

            // WARN: no clue what this is.
            w[col] = Some(row);

            for i in 0..n {
                if i == row {
                    continue;
                }

                let c = matrix[i][col] / matrix[row][col];
                for j in col..=m {
                    matrix[i][j] -= matrix[row][j] * c;
                }
            }

            col += 1;
            row += 1;
        }

        let mut answer = vec![None; m];
        for i in 0..m {
            if answer[i].is_some() {
                continue;
            }

            if let Some(w) = w[i] {
                answer[i] = Some(matrix[w][m] / matrix[w][i]);
            }
        }

        for row in matrix.into_iter() {
            let sum: f64 = (0..m).filter_map(|j| Some(answer[j]? * row[j])).sum();

            if (sum - row[m]).abs() > 10e-6 {
                println!("couldn't find solution");
                dbg!(sum, row[m]);

                return None;
            }
        }

        Some(answer)
    }

    let answer = really_solve(matrix.clone()).expect("some solutions");
    let free_variables = answer
        .iter()
        .enumerate()
        .filter(|(_, answer)| answer.is_none())
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    // Generate constraints for free variables.
    let constraints = free_variables
        .iter()
        .map(|&var| {
            let mut max = f64::MAX;

            for row in &matrix {
                let ans = *row.last().unwrap();
                let coeff = row[var];

                if coeff.abs() <= 10e-6 {
                    continue;
                }

                let sum = row
                    .iter()
                    .take(matrix[0].len() - 1)
                    .enumerate()
                    .filter(|(i, _)| *i != var)
                    .map(|(_, coeff)| *coeff)
                    .sum::<f64>();

                max = max.min((ans - sum) / coeff);
            }

            assert!(max >= 0.);
            (var, 0..=(max.ceil() as usize * 2))
        })
        .collect::<Vec<_>>();

    let mut search = vec![vec![]];
    for &(var, ref range) in &constraints {
        search = search
            .into_iter()
            .flat_map(|nums| {
                range
                    .clone()
                    .map(|i| {
                        let mut nums = nums.clone();
                        nums.push((var, i));
                        nums
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
    }

    let min = search
        .into_iter()
        .filter_map(|nums| {
            let m = matrix[0].len();
            let mut matrix = matrix.clone();
            matrix.extend(nums.into_iter().map(|(i, n)| {
                let mut row = vec![0.0; m];
                row[i] = 1.0;
                *row.last_mut().unwrap() = n as f64;
                row
            }));

            really_solve(matrix)
        })
        .filter_map(|answers| {
            answers
                .iter()
                .map(|n| n.ok_or(()))
                .collect::<Result<Vec<_>, _>>()
                .ok()
        })
        .filter(|answers| answers.iter().all(|n| *n >= 0.))
        .min_by_key(|nums| nums.iter().map(|n| n.round() as u64).sum::<u64>())
        .inspect(|n| {
            dbg!(n);
        });

    match min {
        Some(min) => min.iter().map(|n| n.round() as u64).sum(),
        None => {
            dbg!(matrix);
            dbg!(constraints);
            panic!("rip");
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| line.split_once(' ').unwrap())
            .map(|(goal, rest)| {
                (
                    {
                        let goal = goal.as_bytes();
                        build_usize(goal[1..goal.len() - 1].iter().rev().map(|c| *c == b'#'))
                    },
                    rest.split(' ')
                        .take_while(|s| !s.starts_with('{'))
                        .map(|s| {
                            s[1..s.len() - 1]
                                .split(',')
                                .map(|c| c.parse::<usize>().unwrap())
                                .fold(0usize, |value, i| value | (1 << i))
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .map(|(goal, buttons)| {
                let mut a = VecDeque::from_iter([(0, 0, 0)]);

                while let Some((lights, i, count)) = a.pop_front() {
                    let button = buttons[i];

                    let next = lights ^ button;
                    let next_count = count + 1;

                    if next == goal {
                        return next_count;
                    }

                    if i + 1 >= buttons.len() {
                        continue;
                    }

                    a.push_front((lights, i + 1, count));
                    a.push_back((next, i + 1, next_count));
                }

                unreachable!()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| line.split(' '))
            .map(|mut line| (line.next_back().unwrap(), line.skip(1)))
            .map(|(joltages, buttons)| {
                (
                    joltages[1..joltages.len() - 1]
                        .split(',')
                        .map(|joltage| joltage.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                    buttons
                        .map(|collection| {
                            collection[1..collection.len() - 1]
                                .split(',')
                                .map(|button| button.parse::<usize>().unwrap())
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .map(|(joltages, buttons)| {
                joltages
                    .into_iter()
                    .enumerate()
                    .map(|(i, answer)| {
                        buttons
                            .iter()
                            .map(|button| if button.contains(&i) { 1.0 } else { 0.0 })
                            .chain([answer as f64])
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .map(solve)
            .sum(),
    )

    // Some(
    //     input
    //         .lines()
    //         .map(|line| line.split_once(' ').unwrap().1)
    //         .map(|line| {
    //             (
    //                 {
    //                     let joltages = line.split(' ').next_back().unwrap();
    //                     joltages[1..joltages.len() - 1]
    //                         .split(',')
    //                         .map(|j| j.parse::<usize>().unwrap())
    //                         .enumerate()
    //                         .fold([0; 16], |mut arr, (i, n)| {
    //                             arr[i] = n;
    //                             arr
    //                         })
    //                 },
    //                 line.split(' ')
    //                     .take_while(|s| !s.starts_with('{'))
    //                     .map(|s| {
    //                         s[1..s.len() - 1]
    //                             .split(',')
    //                             .map(|c| c.parse::<usize>().unwrap())
    //                             .collect::<Vec<_>>()
    //                     })
    //                     .collect::<Vec<_>>(),
    //             )
    //         })
    //         .map(|(goal_joltages, buttons)| {
    //             let mut a = VecDeque::from_iter([(0, 0, [0; 16])]);
    //             let mut keys = HashMap::<(usize, [usize; 16]), u64>::new();
    //
    //             while let Some((count, i, joltages)) = a.pop_front() {
    //                 keys.remove(&(i, joltages));
    //
    //                 let next_joltages = {
    //                     let mut joltages = joltages;
    //                     buttons[i].iter().for_each(|&i| joltages[i] += 1);
    //                     joltages
    //                 };
    //
    //                 // If they match, goal found.
    //                 if goal_joltages
    //                     .iter()
    //                     .zip(next_joltages.iter())
    //                     .all(|(goal, joltage)| goal == joltage)
    //                 {
    //                     return count + 1;
    //                 }
    //
    //                 let next_key = (i + 1, joltages);
    //                 if i + 1 < buttons.len()
    //                     && keys
    //                         .get(&next_key)
    //                         .map(|current_count| count < *current_count)
    //                         .unwrap_or(true)
    //                 {
    //                     // Don't push the button
    //                     a.push_front((count, i + 1, joltages));
    //                     keys.insert(next_key, count);
    //                 }
    //
    //                 // If any surpass, abandon.
    //                 let next_key = (i, next_joltages);
    //                 if goal_joltages
    //                     .iter()
    //                     .zip(next_joltages.iter())
    //                     .all(|(goal, joltage)| joltage <= goal)
    //                     && keys
    //                         .get(&next_key)
    //                         .map(|current_count| count + 1 < *current_count)
    //                         .unwrap_or(true)
    //                 {
    //                     // Push the button.
    //                     a.push_back((count + 1, i, next_joltages));
    //                     keys.insert(next_key, count + 1);
    //                 }
    //             }
    //
    //             unreachable!()
    //         })
    //         .inspect(|n| println!("solved: {n}"))
    //         .sum(),
    // )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
        panic!();
    }

    #[test]
    fn matrix() {
        assert_eq!(
            solve(vec![
                vec![0., 0., 0., 0., 1., 1., 3.],
                vec![0., 1., 0., 0., 0., 1., 5.],
                vec![0., 0., 1., 1., 1., 0., 4.],
                vec![1., 1., 0., 1., 0., 0., 7.],
            ]),
            10
        );
    }

    #[test]
    fn matrix2() {
        solve(vec![
            vec![1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 6.0],
            vec![1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 4.0],
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 20.0],
            vec![0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 16.0],
        ]);
    }
}

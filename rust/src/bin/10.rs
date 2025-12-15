use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

advent_of_code::solution!(10);

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

#[derive(Clone, Copy, Debug, Eq)]
struct Number {
    num: usize,
    denom: usize,
    neg: bool,
}
impl Number {
    fn simplify(mut self) -> Self {
        let gcd = gcd(self.num, self.denom);
        self.num /= gcd;
        self.denom /= gcd;
        assert_ne!(self.denom, 0);
        if self.num == 0 {
            self.neg = false;
        }
        self
    }

    fn abs(mut self) -> Self {
        self.neg = false;
        self
    }
}
impl Default for Number {
    fn default() -> Self {
        Self {
            num: 0,
            denom: 1,
            neg: false,
        }
    }
}
impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let denom = lcm(self.denom, rhs.denom);

        let lhs_num = self.num * (denom / self.denom);
        let rhs_num = rhs.num * (denom / rhs.denom);

        let (num, neg) = match ((self.neg, lhs_num), (rhs.neg, rhs_num)) {
            ((sign @ false, lhs), (false, rhs)) | ((sign @ true, lhs), (true, rhs)) => {
                (lhs + rhs, sign)
            }
            ((false, pos), (true, neg)) | ((true, neg), (false, pos)) => {
                (usize::abs_diff(pos, neg), neg > pos)
            }
        };

        Self { num, denom, neg }.simplify()
    }
}
impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for Number {
    type Output = Self;

    fn sub(self, mut rhs: Self) -> Self::Output {
        // Toggle the sign of the rhs.
        rhs.neg = !rhs.neg;

        // Add the operands.
        self.add(rhs)
    }
}
impl SubAssign for Number {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            num: self.num * rhs.num,
            denom: self.denom * rhs.denom,
            neg: self.neg != rhs.neg,
        }
        .simplify()
    }
}
impl MulAssign for Number {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl Div for Number {
    type Output = Self;

    fn div(self, mut rhs: Self) -> Self::Output {
        // Flip the fraction.
        (rhs.num, rhs.denom) = (rhs.denom, rhs.num);

        // Multiply the operands.
        self.mul(rhs)
    }
}
impl DivAssign for Number {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        let lhs = self.simplify();
        let rhs = other.simplify();

        lhs.num == rhs.num && lhs.denom == rhs.denom && lhs.neg == rhs.neg
    }
}
impl PartialEq<usize> for Number {
    fn eq(&self, other: &usize) -> bool {
        let other = Self::from(*other);
        <Self as PartialEq>::eq(self, &other)
    }
}
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialOrd<usize> for Number {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        let other = Self::from(*other);
        <Self as PartialOrd>::partial_cmp(self, &other)
    }
}
impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = self.simplify();
        let rhs = other.simplify();

        match (lhs.neg, rhs.neg) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => {
                let scale = lcm(lhs.denom, rhs.denom);

                let lhs_num = lhs.num * scale;
                let rhs_num = rhs.num * scale;

                let (lhs, rhs) = if lhs.neg {
                    (rhs_num, lhs_num)
                } else {
                    (lhs_num, rhs_num)
                };

                lhs.cmp(&rhs)
            }
        }
    }
}
impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(n) = isize::try_from(*self) {
            return write!(f, "{n}");
        }

        write!(f, "(")?;

        if self.neg {
            write!(f, "-")?;
        }

        write!(f, "{}", self.num)?;

        if self.denom != 1 {
            write!(f, "/{}", self.denom)?;
        }

        write!(f, ")")?;

        Ok(())
    }
}
impl From<usize> for Number {
    fn from(value: usize) -> Self {
        Self {
            num: value,
            denom: 1,
            neg: false,
        }
    }
}
impl TryFrom<Number> for isize {
    type Error = ();

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        let value = value.simplify();

        if value.denom != 1 {
            return Err(());
        }

        Ok(value.num as isize * if value.neg { -1 } else { 1 })
    }
}
impl TryFrom<Number> for usize {
    type Error = ();

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        let value = value.simplify();

        if value.neg || value.denom != 1 {
            return Err(());
        }

        Ok(value.num)
    }
}
impl TryFrom<Number> for u64 {
    type Error = ();

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        Ok(usize::try_from(value)? as u64)
    }
}
impl Sum for Number {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|sum, n| sum + n).unwrap_or(Number::default())
    }
}

#[derive(Clone, Debug)]
struct Matrix {
    matrix: Vec<Vec<Number>>,
}
impl Matrix {
    pub fn new(matrix: Vec<Vec<Number>>) -> Self {
        Self { matrix }
    }

    #[inline(always)]
    pub fn n(&self) -> usize {
        self.matrix.len()
    }

    #[inline(always)]
    pub fn m(&self) -> usize {
        self.matrix[0].len() - 1
    }

    pub fn solve(&self) -> Option<Vec<Option<Number>>> {
        let mut matrix = self.clone();

        let mut col = 0;
        let mut row = 0;

        let mut w = vec![None; matrix.m()];

        while col < matrix.m() && row < matrix.n() {
            // Select the pivot as the largest absolute value in the column.
            let pivot = (row..matrix.n())
                .reduce(|pivot, i| {
                    if matrix.matrix[i][col].abs() > matrix.matrix[pivot][col].abs() {
                        i
                    } else {
                        pivot
                    }
                })
                .unwrap();

            // Cannot pivot this column, advance and try again.
            if matrix.matrix[pivot][col] == 0 {
                col += 1;
                continue;
            }

            // Pivot the row.
            matrix.matrix.swap(row, pivot);

            // Record which row the column pivoted to.
            w[col] = Some(row);

            let divisor = matrix.matrix[row][col];
            if divisor != 0 {
                for c in &mut matrix.matrix[row] {
                    *c /= divisor;
                }
            }

            for i in 0..matrix.n() {
                if i == row {
                    continue;
                }

                let c = matrix.matrix[i][col];
                for j in col..=matrix.m() {
                    matrix.matrix[i][j] = matrix.matrix[i][j] - (matrix.matrix[row][j] * c);
                }
            }

            col += 1;
            row += 1;
        }

        let answers = (0..matrix.m())
            .map(|i| {
                Some({
                    let w = w[i]?;

                    matrix.matrix[w][matrix.m()] / matrix.matrix[w][i]
                })
            })
            .collect::<Vec<_>>();

        matrix.verify(&answers).then_some(answers)
    }

    fn verify(&self, answers: &[Option<Number>]) -> bool {
        self.matrix.iter().all(|row| {
            row[0..self.m()]
                .iter()
                .zip(answers)
                .filter_map(|(n, answer)| Some(*n * (*answer)?))
                .sum::<Number>()
                == row[self.m()]
        })
    }

    fn free_variables(&self) -> impl Iterator<Item = usize> {
        self.solve()
            .unwrap()
            .into_iter()
            .enumerate()
            .filter(|(_, answer)| answer.is_none())
            .map(|(i, _)| i)
    }
}
impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.matrix {
            for c in &row[0..row.len() - 1] {
                write!(f, "{c} ")?;
            }

            writeln!(f, "| {}", row[row.len() - 1])?
        }

        Ok(())
    }
}

fn solve_min_vars(matrix: &Matrix) -> Number {
    if let Some(answer) = matrix.solve().and_then(|answer| {
        let mut a = Vec::with_capacity(answer.len());
        for answer in answer {
            a.push(answer?);
        }
        Some(a)
    }) {
        return answer.into_iter().sum();
    }

    let free_variables = matrix.free_variables().collect::<Vec<_>>();
    let constraints = free_variables
        .iter()
        .map(|var| (var, generate_max_constraint(*var, matrix)))
        .collect::<Vec<_>>();

    let mut search = vec![vec![]];
    for (&var, ref max) in constraints {
        search = search
            .into_iter()
            .flat_map(|nums| {
                (0..)
                    .map(Number::from)
                    .take_while(|n| n <= max)
                    .map(|n| {
                        let mut nums = nums.clone();
                        nums.push((var, n));
                        nums
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
    }

    let mut solver = matrix.clone();
    let solution_rows = free_variables
        .iter()
        .map(|_| {
            let i = solver.matrix.len();
            solver.matrix.push(vec![0.into(); solver.matrix[0].len()]);
            i
        })
        .collect::<Vec<_>>();

    let m = solver.m();
    search
        .into_iter()
        .flat_map(|s| {
            for ((free_variable, answer), &row) in s.iter().zip(solution_rows.iter()) {
                solver.matrix[row][*free_variable] = 1.into();
                solver.matrix[row][m] = *answer;
            }

            let answers = solver.solve()?;
            if !matrix.verify(&answers) {
                return None;
            }

            let answers = answers
                .into_iter()
                .flatten()
                .filter(|n| n >= &0 && n.denom == 1)
                .collect::<Vec<_>>();

            if answers.len() != m {
                return None;
            }

            Some(answers)
        })
        .map(|answers| (&answers, answers.iter().cloned().sum::<Number>()).1)
        .filter(|answer| answer.denom == 1)
        .min()
        .or_else(|| {
            println!("{matrix}");
            None
        })
        .unwrap()
}

fn generate_max_constraint(free_var: usize, matrix: &Matrix) -> Number {
    let max = matrix
        .matrix
        .iter()
        .filter_map(|row| {
            let ans = *row.last().unwrap();
            let coeff = row[free_var];

            if coeff == 0 {
                return None;
            }

            // let sum = row[0..matrix.m()]
            //     .iter()
            //     .enumerate()
            //     .filter(|(i, _)| *i != free_var)
            //     .map(|(_, coeff)| *coeff)
            //     .sum::<Number>();

            Some(ans / coeff)
        })
        .max()
        .expect("bounded variable");

    assert!(max >= 0);

    max
}

fn build_usize(iter: impl Iterator<Item = bool>) -> usize {
    iter.fold(0, |value, b| (value << 1) | (b as usize) & 1)
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
                Matrix::new(
                    joltages
                        .into_iter()
                        .enumerate()
                        .map(|(i, answer)| {
                            buttons
                                .iter()
                                .map(|button| if button.contains(&i) { 1 } else { 0 })
                                .chain([answer])
                                .map(Number::from)
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .map(|matrix| solve_min_vars(&matrix))
            .sum::<Number>()
            .try_into()
            .unwrap(),
    )
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
    }

    #[test]
    fn matrix3() {
        let matrix = Matrix::new(vec![
            vec![
                1.into(),
                1.into(),
                0.into(),
                0.into(),
                0.into(),
                1.into(),
                6.into(),
            ],
            vec![
                1.into(),
                1.into(),
                1.into(),
                1.into(),
                0.into(),
                0.into(),
                4.into(),
            ],
            vec![
                0.into(),
                0.into(),
                0.into(),
                0.into(),
                1.into(),
                1.into(),
                20.into(),
            ],
            vec![
                0.into(),
                1.into(),
                1.into(),
                0.into(),
                1.into(),
                0.into(),
                16.into(),
            ],
        ]);

        assert_eq!(solve_min_vars(&matrix), 24);
    }

    mod number {
        use super::*;

        #[test]
        fn add() {
            assert_eq!(
                Number {
                    num: 3,
                    denom: 7,
                    neg: false
                } + Number {
                    num: 8,
                    denom: 3,
                    neg: false
                },
                Number {
                    num: 65,
                    denom: 21,
                    neg: false
                }
            );
        }

        #[test]
        fn sub() {
            assert_eq!(
                Number {
                    num: 1,
                    denom: 4,
                    neg: false
                } - Number {
                    num: 1,
                    denom: 8,
                    neg: false
                },
                Number {
                    num: 1,
                    denom: 8,
                    neg: false
                }
            )
        }
    }
}

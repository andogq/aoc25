advent_of_code::solution!(6);

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
}

fn parse(input: &str) -> impl Iterator<Item = (Operation, impl Iterator<Item = u64>)> {
    let height = input.lines().count() - 1;
    let width = input.char_indices().find(|(_, c)| *c == '\n').unwrap().0 + 1;

    let operation_offset = width * height;

    (0..).scan(0, move |column_offset, _| {
        // Save the current offset for processing
        let current_offset = *column_offset;

        // Extract out the operation.
        let mut operations = input[operation_offset + *column_offset..].char_indices();
        let operation = operations.next().map(|(_, c)| match c {
            '+' => Operation::Add,
            '*' => Operation::Multiply,
            c => panic!("expected operation, but found {c}"),
        })?;
        let column_width = operations
            .take_while(|(_, c)| c.is_ascii_whitespace())
            .last()
            .map(|(width, _)| width + 1)
            .unwrap();

        // Update for the next offset.
        *column_offset += column_width;

        let values = (0..height)
            .map(move |i| (i * width) + current_offset)
            .map(move |start| input[start..start + column_width].trim())
            .map(|s| s.parse().unwrap());

        Some((operation, values))
    })
}

fn parse2(input: &str) -> impl Iterator<Item = (Operation, impl Iterator<Item = u64>)> {
    let height = input.lines().count() - 1;
    let width = input.char_indices().find(|(_, c)| *c == '\n').unwrap().0 + 1;

    let operation_offset = width * height;

    (0..).scan(0, move |column_offset, _| {
        // Save the current offset for processing
        let current_offset = *column_offset;

        // Extract out the operation.
        let mut operations = input[operation_offset + *column_offset..].char_indices();
        let operation = operations.next().map(|(_, c)| match c {
            '+' => Operation::Add,
            '*' => Operation::Multiply,
            c => panic!("expected operation, but found {c}"),
        })?;
        let column_width = operations
            .take_while(|(_, c)| c.is_ascii_whitespace())
            .last()
            .map(|(width, _)| width + 1)
            .unwrap();

        // Update for the next offset.
        *column_offset += column_width;

        let values = (0..column_width - 1)
            .rev()
            .map(move |i| i + current_offset)
            .map(move |i| {
                (0..height)
                    .scan(input.chars().skip(i), |chars, _| {
                        let n = chars.next().unwrap();

                        // HACK: Advance the iterator.
                        (0..width - 1).for_each(|_| {
                            chars.next().unwrap();
                        });

                        Some(n)
                    })
                    .flat_map(|c| match c {
                        c @ '0'..='9' => Some(c.to_digit(10).unwrap() as u64),
                        ' ' => None,
                        c => panic!("unknown digit: {c}"),
                    })
                    .reduce(|total, n| (total * 10) + n)
                    .unwrap_or(0)
            });

        Some((operation, values))
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .map(|(operation, values)| match operation {
                Operation::Add => values.sum(),
                Operation::Multiply => values.reduce(|total, n| total * n).unwrap_or(0),
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse2(input)
            .map(|(operation, values)| match operation {
                Operation::Add => values.sum(),
                Operation::Multiply => values.reduce(|total, n| total * n).unwrap_or(0),
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}

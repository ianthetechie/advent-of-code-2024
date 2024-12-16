use ndarray::{Array, Axis};
use std::collections::HashSet;

const TEST_INPUT: &str = include_str!("test.txt");
const FULL_INPUT: &str = include_str!("full.txt");

fn part1(input: &str) -> usize {
    let line_length = input.lines().next().unwrap().len();
    let num_lines = input.lines().count();
    let flat_stripped = input.replace("\n", "").chars().collect::<Vec<_>>();
    let mut arr = Array::from_shape_vec((num_lines, line_length), flat_stripped).unwrap();

    let col_sum = arr
        .columns()
        .into_iter()
        .map(|col| {
            let s = col.iter().collect::<String>();
            s.matches("XMAS").count() + s.matches("SAMX").count()
        })
        .sum::<usize>();

    let down_diag_sum = FwdDownDiagonalIterator::new(num_lines, line_length)
        .map(|indices| {
            let text = indices
                .iter()
                .map(|pos| arr[[pos.row, pos.column]])
                .collect::<String>();
            text.matches("XMAS").count() + text.matches("SAMX").count()
        })
        .sum::<usize>();

    // Because I was too lazy to write two diagonal iterators...
    arr.invert_axis(Axis(1));

    let up_diag_sum = FwdDownDiagonalIterator::new(num_lines, line_length)
        .map(|indices| {
            let text = indices
                .iter()
                .map(|pos| arr[[pos.row, pos.column]])
                .collect::<String>();
            text.matches("XMAS").count() + text.matches("SAMX").count()
        })
        .sum::<usize>();

    // Horizontal fwd and rev are easy; input has line breaks
    input.matches("XMAS").count()
        + input.matches("SAMX").count()
        + col_sum
        + down_diag_sum
        + up_diag_sum
}

fn part2(input: &str) -> usize {
    let line_length = input.lines().next().unwrap().len();
    let num_lines = input.lines().count();
    let flat_stripped = input.replace("\n", "").chars().collect::<Vec<_>>();
    let mut arr = Array::from_shape_vec((num_lines, line_length), flat_stripped).unwrap();

    let down_diag_matches: HashSet<_> = FwdDownDiagonalIterator::new(num_lines, line_length)
        .flat_map(|indices| {
            let text = indices
                .iter()
                .map(|pos| arr[[pos.row, pos.column]])
                .collect::<String>();
            text.match_indices("MAS")
                .map(|(i, _)| indices[i + 1])
                .chain(text.match_indices("SAM").map(|(i, _)| indices[i + 1]))
                .collect::<Vec<_>>()
        })
        .collect();

    // Mind you that we're reversing every row in the array, which will mess with positioning...
    arr.invert_axis(Axis(1));
    let transpose_row = |pos: Position| Position {
        row: pos.row,
        column: line_length - pos.column - 1,
    };

    let up_diag_matches: HashSet<_> = FwdDownDiagonalIterator::new(num_lines, line_length)
        .flat_map(|indices| {
            let text = indices
                .iter()
                .map(|pos| arr[[pos.row, pos.column]])
                .collect::<String>();
            text.match_indices("MAS")
                .map(|(i, _)| transpose_row(indices[i + 1]))
                .chain(
                    text.match_indices("SAM")
                        .map(|(i, _)| transpose_row(indices[i + 1])),
                )
                .collect::<Vec<_>>()
        })
        .collect();

    eprintln!("down: {:?}\nup: {:?}", down_diag_matches, up_diag_matches);
    down_diag_matches.intersection(&up_diag_matches).count()
}

fn main() {
    println!(
        "P1 - test: {} full: {}",
        part1(TEST_INPUT),
        part1(FULL_INPUT)
    );

    println!(
        "P2 - test: {} full: {}",
        part2(TEST_INPUT),
        part2(FULL_INPUT)
    );
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

fn diagonal_down_from_position(start: Position, length: usize) -> Vec<Position> {
    (0..length)
        .map(|i| Position {
            row: start.row + i,
            column: start.column + i,
        })
        .collect()
}

/// An iterator over all possible diagonals (not just the one from the origin)
/// going left to right and downward.
struct FwdDownDiagonalIterator {
    n_rows: usize,
    n_cols: usize,
    start_position: Position,
}

impl FwdDownDiagonalIterator {
    fn new(n_rows: usize, n_cols: usize) -> Self {
        Self {
            n_rows,
            n_cols,
            start_position: Position {
                row: n_rows, // NB: One past the end
                column: 0,
            },
        }
    }
}

impl Iterator for FwdDownDiagonalIterator {
    type Item = Vec<Position>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_position.row > 0 {
            // First, rows bottom up
            self.start_position.row -= 1;

            let length = (self.n_rows - self.start_position.row).min(self.n_cols);
            Some(diagonal_down_from_position(self.start_position, length))
        } else if self.start_position.column < self.n_cols - 1 {
            // Then, columns left to right
            self.start_position.column += 1;

            let length = (self.n_cols - self.start_position.column).min(self.n_rows);
            Some(diagonal_down_from_position(self.start_position, length))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FwdDownDiagonalIterator, Position};
    use ndarray::arr2;

    #[test]
    fn test_diagonal_iterator_indices() {
        let iter = FwdDownDiagonalIterator::new(3, 3);
        let collected: Vec<_> = iter.collect();
        assert_eq!(
            collected,
            vec![
                // Lower triangle
                vec![Position { row: 2, column: 0 }],
                vec![
                    Position { row: 1, column: 0 },
                    Position { row: 2, column: 1 }
                ],
                // Diagonal
                vec![
                    Position { row: 0, column: 0 },
                    Position { row: 1, column: 1 },
                    Position { row: 2, column: 2 }
                ],
                // Upper triangle
                vec![
                    Position { row: 0, column: 1 },
                    Position { row: 1, column: 2 }
                ],
                vec![Position { row: 0, column: 2 }],
            ]
        );
    }

    #[test]
    fn test_diagonal_iterator_ndarray() {
        let iter = FwdDownDiagonalIterator::new(3, 3);
        let arr = arr2(&[['a', 'b', 'c'], ['d', 'e', 'f'], ['g', 'h', 'i']]);

        let result = iter
            .map(|indices| {
                indices
                    .iter()
                    .map(|pos| arr[[pos.row, pos.column]])
                    .collect::<String>()
            })
            .collect::<Vec<_>>();
        assert_eq!(result, vec!["g", "dh", "aei", "bf", "c"]);
    }
}

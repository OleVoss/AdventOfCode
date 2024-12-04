use itertools::Itertools;
use tracing::info;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut xmas_hits: Vec<XmasHit> = Vec::new();

    let grid = parse_grid(input)?;
    let mut col_index = 0;
    while col_index + 4 <= grid[0].len() {
        let mut row_index = 0;
        while row_index + 4 <= grid.len() {
            let square: Vec<[char; 4]> = (0..4)
                .into_iter()
                .map(|i| {
                    let row: [char; 4] = [
                        grid[row_index + i][col_index],
                        grid[row_index + i][col_index + 1],
                        grid[row_index + i][col_index + 2],
                        grid[row_index + i][col_index + 3],
                    ];
                    return row;
                })
                .collect();
            xmas_hits.append(&mut find_in_square(square, row_index, col_index)?);
            row_index += 1;
        }
        col_index += 1;
    }
    xmas_hits.sort();
    xmas_hits.dedup();
    let xmas_count = xmas_hits.len();
    Ok(xmas_count.to_string())
}

// Patterns
const XMAS_NORMAL: [char; 4] = ['X', 'M', 'A', 'S'];
const XMAS_REVERSE: [char; 4] = ['S', 'A', 'M', 'X'];

#[derive(Debug, PartialOrd, Ord)]
struct XmasHit {
    row: usize,
    col: usize,
    orient: Orientation,
    direction: Direction,
}

impl XmasHit {
    pub fn new(row: usize, col: usize, or: Orientation, dir: Direction) -> Self {
        Self {
            row,
            col,
            orient: or,
            direction: dir,
        }
    }
}

impl PartialEq for XmasHit {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
            && self.col == other.col
            && self.orient == other.orient
            && self.direction == other.direction
    }
}

impl Eq for XmasHit {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Orientation {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Normal,
    Reverse,
}

fn find_in_square(
    square: Vec<[char; 4]>,
    row: usize,
    column: usize,
) -> miette::Result<Vec<XmasHit>> {
    let mut hits: Vec<XmasHit> = Vec::new();
    dbg!("New finder::\n");
    dbg!(row, column, &square);
    for i in 0..4 {
        // rows
        if square[i] == XMAS_NORMAL {
            hits.push(XmasHit::new(
                row,
                column,
                Orientation::Horizontal,
                Direction::Normal,
            ));
        } else if square[i] == XMAS_REVERSE {
            hits.push(XmasHit::new(
                row,
                column,
                Orientation::Horizontal,
                Direction::Reverse,
            ));
        }

        // cols
        let col: Vec<char> = square.iter().map(|r| r[i]).collect();
        if col == XMAS_NORMAL {
            hits.push(XmasHit::new(
                row,
                column,
                Orientation::Vertical,
                Direction::Normal,
            ));
        } else if col == XMAS_REVERSE {
            hits.push(XmasHit::new(
                row,
                column,
                Orientation::Vertical,
                Direction::Reverse,
            ));
        }
    }

    // diagonal
    for direction in [0, 3] {
        let dia: Vec<char> = (0..4)
            .into_iter()
            .map(|i| square[i][(direction as i32 - i as i32).abs() as usize])
            .collect();

        if dia.as_slice() == XMAS_NORMAL {
            hits.push(XmasHit::new(
                row,
                column,
                Orientation::Diagonal,
                Direction::Normal,
            ));
        } else if dia.as_slice() == XMAS_REVERSE {
            hits.push(XmasHit::new(
                row,
                column,
                Orientation::Diagonal,
                Direction::Reverse,
            ));
        }
    }
    dbg!(&hits);
    Ok(hits)
}

fn parse_grid(input: &str) -> miette::Result<Vec<Vec<char>>> {
    let mut outer: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        outer.push(row);
    }

    Ok(outer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() -> miette::Result<()> {
        let input = vec![
            ['X', 'M', 'A', 'S'],
            ['M', 'M', '=', 'A'],
            ['A', 'M', 'A', 'M'],
            ['S', 'A', 'M', 'S'],
        ];

        let hits = find_in_square(input, 0, 0)?;
        let expect = vec![
            XmasHit {
                row: 0,
                col: 0,
                orient: Orientation::Horizontal,
                direction: Direction::Normal,
            },
            XmasHit {
                row: 0,
                col: 0,
                orient: Orientation::Vertical,
                direction: Direction::Normal,
            },
            XmasHit {
                row: 0,
                col: 0,
                orient: Orientation::Diagonal,
                direction: Direction::Normal,
            },
        ];
        assert_eq!(hits, expect);
        Ok(())
    }

    #[test]
    fn test_build_grid() -> miette::Result<()> {
        let input = "XMAS\nSMAS\nAMSA\nXMAS";

        let expect = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['S', 'M', 'A', 'S'],
            vec!['A', 'M', 'S', 'A'],
            vec!['X', 'M', 'A', 'S'],
        ];
        assert_eq!(parse_grid(input)?, expect);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "XMXMAS
XMSMSA
MXXAMA
MXXAMA
AXMASX";
        assert_eq!(process(input)?, "18");

        Ok(())
    }
}

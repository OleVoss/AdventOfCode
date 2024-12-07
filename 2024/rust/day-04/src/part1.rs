#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut xmas_count = 0;

    let grid = parse_grid(input)?;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'X' {
                let hits: u32 = DIRECTIONS
                    .iter()
                    .map(|dir_vec| {
                        let word: String = dir_vec
                            .iter()
                            .map(|pos| {
                                let x = (row as i32) + pos.0;
                                let y = (col as i32) + pos.1;
                                match grid.get(x as usize) {
                                    Some(r) => match r.get(y as usize) {
                                        Some(c) => c,
                                        None => &'.',
                                    },
                                    None => &'.',
                                }
                            })
                            .collect();
                        if word == "MAS" {
                            1
                        } else {
                            0
                        }
                    })
                    .sum();
                xmas_count += hits;
            }
        }
    }

    Ok(xmas_count.to_string())
}

// Patterns
const DIRECTIONS: [[(i32, i32); 3]; 8] = [
    [(-1, 0), (-2, 0), (-3, 0)],
    [(-1, 1), (-2, 2), (-3, 3)],
    [(0, 1), (0, 2), (0, 3)],
    [(1, 1), (2, 2), (3, 3)],
    [(1, 0), (2, 0), (3, 0)],
    [(1, -1), (2, -2), (3, -3)],
    [(0, -1), (0, -2), (0, -3)],
    [(-1, -1), (-2, -2), (-3, -3)],
];

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
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(process(input)?, "18");

        Ok(())
    }
}

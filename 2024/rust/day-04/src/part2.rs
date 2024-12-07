#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut x_mas_count = 0;

    let grid = parse_grid(input)?;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'A' {
                let word: String = DIRECTIONS
                    .iter()
                    .map(|pos| {
                        let x = row as i32 + pos.0;
                        let y = col as i32 + pos.1;
                        match grid.get(x as usize) {
                            Some(r) => r.get(y as usize).unwrap_or(&'.'),
                            None => &'.',
                        }
                    })
                    .collect();
                if (word == "MSSM")
                    || (word == "MMSS")
                    || (word == "SMMS")
                    || (word == "MSMS")
                    || (word == "SMSM")
                    || (word == "SSMM")
                {
                    x_mas_count += 1;
                }
            }
        }
    }

    Ok(x_mas_count.to_string())
}

// Patterns
const DIRECTIONS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];

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
        assert_eq!(process(input)?, "9");

        Ok(())
    }
}

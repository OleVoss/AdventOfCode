#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut xmas_count = 0;

    let grid = parse_grid(input)?;
    let mut col_index = 0;
    while col_index + 4 < grid[0].len() {
        let mut row_index = 0;
        while row_index + 4 < grid.len() {
            let square: Vec<&[char]> = (0..4).into_iter().map(|i| &grid[i][0..4]).collect();
            xmas_count += find_in_square(square)?;
            row_index += 1;
        }
    }

    Ok(xmas_count.to_string())
}

// Patterns
const XMAS_NORMAL: [char; 4] = ['X', 'M', 'A', 'S'];
const XMAS_REVERSE: [char; 4] = ['S', 'A', 'M', 'X'];

fn find_in_square(square: Vec<&[char]>) -> miette::Result<i32> {
    let mut count = 0;

    for i in 0..4 {
        // rows
        if square[i] == XMAS_NORMAL || square[i] == XMAS_REVERSE {
            count += 1;
        }

        // cols
        let col: Vec<char> = square.iter().map(|r| r[i]).collect();
        if col == XMAS_NORMAL || col == XMAS_REVERSE {
            count += 1;
        }
    }

    // diagonal
    for direction in [0, 3] {
        let dia: Vec<char> = (0..4)
            .into_iter()
            .map(|i| square[i][(direction as i32 - i as i32).abs() as usize])
            .collect();

        if dia.as_slice() == XMAS_NORMAL || dia.as_slice() == XMAS_REVERSE {
            count += 1;
        }
    }

    Ok(count)
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
        let input = [
            ['X', 'M', 'A', 'S'],
            ['M', 'M', '=', 'A'],
            ['A', 'M', 'A', 'M'],
            ['S', 'A', 'M', 'S'],
        ];
        assert_eq!(3, find_in_square(&input)?);
        Ok(())
    }

    fn test_build_grid() -> miette::Result<()> {
        let input = "XMAS
        SMAS
        AMSA";

        let expect = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['S', 'M', 'A', 'S'],
            vec!['A', 'M', 'A', 'S'],
        ];
        assert_eq!(parse_grid(input)?, expect);
        Ok(())
    }
}

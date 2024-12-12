#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("day 01 - part 1");
}

#[derive(Debug, Default, Clone)]
struct Field {
    character: char,
    grid: Vec<Vec<char>>,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.character == other.character
    }
}

fn parse(input: &str) -> miette::Result<Vec<Field>> {
    let mut fields: Vec<Field> = Vec::new();

    let mut last_line_nothing = true;

    let mut current_plant: char = '.';
    let mut current_field: Field = Field::default();
    for (row, line) in input.lines().enumerate() {
        last_line_nothing = true;

        for (col, plant) in line.chars().enumerate() {
            if plant != current_plant {
                current_plant = plant;
                current_field = Field {
                    character: current_plant,
                    grid: vec![Vec::default()],
                };
                current_field.grid[0].push(current_plant);
            } else {
                last_line_nothing = false;
                current_field.grid[row].push(plant);
            }
        }

        if last_line_nothing == true {
            fields.push(current_field.clone());
        }
    }

    Ok(fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let fields = parse(input)?;

        dbg!(fields);

        assert!(false);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("", process(input)?);
        Ok(())
    }
}

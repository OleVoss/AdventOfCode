use nom::{
    bytes::complete::{tag, take_until, take_until1},
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

use miette::miette;

fn main() -> miette::Result<()> {
    let content = include_str!("../input.txt");

    let (_input, instructions) = parse(content).map_err(|e| miette!("parse failed {}", e))?;

    let result: u32 = instructions
        .iter()
        .map(|ins| match ins {
            Instruction::Mul(a, b) => a * b,
        })
        .sum();

    println!("sum of mul: {result}");
    Ok(())
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, parse_multiplication).map(|(_discard, ins)| ins))(input)
}

fn parse_multiplication(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Mul(u32, u32),
}

#[cfg(test)]
mod test {
    use miette::IntoDiagnostic;

    use crate::{parse, parse_multiplication, Instruction};

    #[test]
    fn test_mul_parser() {
        let test_str = "mul(1,4)";
        let (_, f) = parse_multiplication(test_str).unwrap();
        assert_eq!(f, Instruction::Mul(1, 4));
    }

    fn test_many() -> miette::Result<()> {
        let test_str = "mulxmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected = vec![
            Instruction::Mul(2, 4),
            Instruction::Mul(5, 5),
            Instruction::Mul(11, 8),
            Instruction::Mul(8, 5),
        ];
        let (_, ins) = parse(test_str).into_diagnostic()?;
        assert_eq!(ins, expected);
        Ok(())
    }
}

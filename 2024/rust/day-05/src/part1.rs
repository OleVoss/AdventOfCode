use miette::IntoDiagnostic;
use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::separated_pair,
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("day 01 - part 1");
}

#[derive(Debug, PartialEq)]
struct PagePair {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
struct ManualUpdate {
    pub pages: Vec<i32>,
}

fn parse(input: &str) -> IResult<&str, Vec<PagePair>> {
    let (rest, pairs) =
        many1(many_till(anychar, parse_page_pair).map(|(_discard, pair)| pair))(input)?;
    Ok((rest, pairs))
}

fn parse_page_pair(input: &str) -> IResult<&str, PagePair> {
    let (input, pair) = separated_pair(complete::u32, tag("|"), complete::u32)(input)?;
    Ok((
        input,
        PagePair {
            x: pair.0,
            y: pair.1,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_paige_pair() -> miette::Result<()> {
        let input = "68|13";
        let (_, pair) = parse_page_pair(input).into_diagnostic()?;

        assert_eq!(pair, PagePair { x: 68, y: 13 });

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let page_pairs = parse(TEST_INPUT).into_diagnostic()?;
        dbg!(page_pairs);
        panic!();
        Ok(())
    }

    const TEST_INPUT: &str = "47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13
    
    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47
    ";
}

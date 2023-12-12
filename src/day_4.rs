use log::debug;
use pest::Parser;
use pest_derive::Parser;
use std::num::ParseIntError;

#[derive(Parser)]
#[grammar = "day_4.pest"]
struct IdentParser;

#[derive(Debug)]
pub struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn worth(self: &Card) -> u32 {
        let intersection: Vec<u32> = self
            .numbers
            .clone()
            .into_iter()
            .filter(|num| self.winning_numbers.contains(num))
            .collect();
        debug!("{:?}", intersection);
        if intersection.is_empty() {
            return 0;
        };
        let exp: u32 = (intersection.len() as u32) - 1;
        2u32.pow(exp)
    }
}

pub fn parse(input: &str) -> Result<Vec<Card>, ParseIntError> {
    let mut results: Vec<Card> = vec![];

    let cards = IdentParser::parse(Rule::cards, input).unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for card in cards {
        let mut winning_numbers: Vec<u32> = vec![];
        let mut numbers: Vec<u32> = vec![];

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in card.into_inner() {
            match inner_pair.as_rule() {
                Rule::winning_number => {
                    winning_numbers.push(inner_pair.as_str().parse::<u32>()?);
                }
                Rule::number => {
                    numbers.push(inner_pair.as_str().parse::<u32>()?);
                }
                _ => unreachable!(),
            };
        }
        results.push(Card {
            winning_numbers,
            numbers,
        });
    }

    Ok(results)
}

pub mod part_one {
    use super::*;
    pub fn sum_worths(input: &str) -> u32 {
        debug!("{:?}", input);
        if let Ok(cards) = parse(input) {
            return cards.iter().map(|card| card.worth()).sum();
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

    #[test]
    fn test_pest() {
        assert_eq!(part_one::sum_worths(INPUT.trim()), 13);
    }
}

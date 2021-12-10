use std::collections::LinkedList;
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Eq)]
enum BracketType {
    BRACE,
    ROUND,
    SQUARE,
    ANGLE,
}

#[derive(Debug, PartialEq, Eq)]
enum Bracket {
    Open(BracketType),
    Close(BracketType),
}

impl TryFrom<char> for Bracket {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '{' => Ok(Self::Open(BracketType::BRACE)),
            '(' => Ok(Self::Open(BracketType::ROUND)),
            '[' => Ok(Self::Open(BracketType::SQUARE)),
            '<' => Ok(Self::Open(BracketType::ANGLE)),

            '}' => Ok(Self::Close(BracketType::BRACE)),
            ')' => Ok(Self::Close(BracketType::ROUND)),
            ']' => Ok(Self::Close(BracketType::SQUARE)),
            '>' => Ok(Self::Close(BracketType::ANGLE)),

            _ => Err(()),
        }
    }
}

fn format(data: &mut impl Read) -> Vec<Vec<Bracket>> {
    {
        let mut s = String::new();
        data.read_to_string(&mut s);
        s
    }
    .split_whitespace()
    .map(|s| s.chars().map(|c| Bracket::try_from(c).unwrap()).collect())
    .collect()
}

pub fn solution_part1(mut data: File) -> u64{
    let data = format(&mut data);

    data.iter()
        .filter_map(|row| {
            let mut items = Vec::with_capacity(row.len());

            for item in row.iter() {
                if let Bracket::Open(opening) = item {
                    items.push(opening);
                } else if let Bracket::Close(closing) = item {
                    let opening = items.pop();
                    if opening != Some(closing) && opening != None {
                        return Some(closing);
                    }
                }
            }

            None
        })
        .map(|delim| match delim {
            BracketType::BRACE => 1197,
            BracketType::ROUND => 3,
            BracketType::SQUARE => 57,
            BracketType::ANGLE => 25137,
        })
        .sum()
}

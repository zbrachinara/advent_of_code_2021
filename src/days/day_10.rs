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

fn find_corrupted(row: &Vec<Bracket>) -> Option<&BracketType> {
    let mut items = Vec::with_capacity(row.len());

    for item in row.iter() {
        if let Bracket::Open(opening) = item {
            items.push(opening);
        } else if let Bracket::Close(closing) = item {
            let opening = items.pop();
            if opening != Some(closing) && opening != None {
                return Some(&closing);
            }
        }
    }

    None
}

pub fn solution_part1(mut data: File) -> u64 {
    let data = format(&mut data);

    data.iter()
        .filter_map(find_corrupted)
        .map(|delim| match delim {
            BracketType::BRACE => 1197,
            BracketType::ROUND => 3,
            BracketType::SQUARE => 57,
            BracketType::ANGLE => 25137,
        })
        .sum()
}

pub fn solution_part2(mut data: File) -> u64 {
    let mut scores = format(&mut data)
        .iter()
        .filter(|row| matches!(find_corrupted(row), None))
        // find symbols required to autocomplete
        .map(|row| {
            let mut items = Vec::with_capacity(row.len());

            row.iter().for_each(|item| match item {
                Bracket::Open(i) => items.push(i),
                Bracket::Close(_) => {
                    items.pop();
                }
            });

            items
        })
        .inspect(|s| println!("{:?}", s))
        // find score of autocompletion
        .map(|completion| {
            completion.iter().rev().fold(0u64, |acc, item| {
                dbg!(acc, item);
                acc * 5 + match item {
                    BracketType::BRACE => 3,
                    BracketType::ROUND => 1,
                    BracketType::SQUARE => 2,
                    BracketType::ANGLE => 4,
                }
            })
        })
        .collect::<Vec<_>>();

    let median_position = scores.len() / 2;
    let (_, out, _) = scores.as_mut_slice().select_nth_unstable(median_position);

    *out
}

use anyhow::Context;
use itertools::Itertools;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let part1: i64 = input
        .lines()
        .map(|l| {
            let mut stack = Vec::new();
            l.chars()
                .map(|c| match c {
                    '(' | '[' | '<' | '{' => {
                        stack.push(c);
                        0
                    }
                    ')' | ']' | '>' | '}' => {
                        let expected = stack.pop();
                        match (c, expected) {
                            (')', Some(e)) if e != '(' => 3,
                            (']', Some(e)) if e != '[' => 57,
                            ('}', Some(e)) if e != '{' => 1197,
                            ('>', Some(e)) if e != '<' => 25137,
                            _ => 0,
                        }
                    }
                    _ => 0,
                })
                .sum::<i64>()
        })
        .sum();

    dbg!(&part1);

    let part2 = input
        .lines()
        .flat_map(|l| {
            let mut stack = Vec::new();
            let score = l
                .chars()
                .map(|c| match c {
                    '(' | '[' | '<' | '{' => {
                        stack.push(c);
                        0
                    }
                    ')' | ']' | '>' | '}' => {
                        let expected = stack.pop();
                        match (c, expected) {
                            (')', Some(e)) if e != '(' => 3,
                            (']', Some(e)) if e != '[' => 57,
                            ('}', Some(e)) if e != '{' => 1197,
                            ('>', Some(e)) if e != '<' => 25137,
                            _ => 0,
                        }
                    }
                    _ => 0,
                })
                .sum::<i64>();
            if score == 0 {
                Some(stack.iter().rev().fold(0i64, |acc, c| {
                    acc * 5
                        + match c {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => unreachable!(),
                        }
                }))
            } else {
                None
            }
        })
        .sorted()
        .collect_vec();
    let part2 = part2[part2.len() / 2];
    dbg!(&part2);

    Ok(())
}

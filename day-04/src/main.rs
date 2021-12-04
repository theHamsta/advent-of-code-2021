#![feature(bool_to_option)]

use std::collections::HashSet;

use anyhow::Context;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

fn has_won(block: &Vec<Vec<i64>>, drawn: &HashSet<i64>) -> Option<i64> {
    let height = block.len();
    let width = block[0].len();
    for x in 0..width {
        let mut found = true;
        for y in 0..height {
            let current = block[y][x];
            found &= drawn.contains(&current);
        }
        if found {
            let sum = block.iter().flatten().filter(|x| !drawn.contains(x)).sum();
            return Some(sum);
        }
    }
    for y in 0..height {
        let mut found = true;
        for x in 0..width {
            let current = block[y][x];
            found &= drawn.contains(&current);
        }
        if found {
            let sum = block.iter().flatten().filter(|x| !drawn.contains(x)).sum();
            return Some(sum);
        }
    }
    None
}

fn play1(numbers: &Vec<i64>, blocks: &Vec<Vec<Vec<i64>>>) -> Option<i64> {
    let mut drawn = HashSet::new();
    for draw in numbers {
        drawn.insert(*draw);
        for b in blocks.iter() {
            if let Some(sum) = has_won(b, &drawn) {
                return Some(sum * draw);
            }
        }
    }
    None
}

fn play2(numbers: &Vec<i64>, blocks: &Vec<Vec<Vec<i64>>>) -> Option<i64> {
    let mut drawn = HashSet::new();
    let mut winners = HashSet::new();

    for draw in numbers {
        drawn.insert(*draw);
        for b in blocks.iter() {
            if let Some(sum) = has_won(b, &drawn) {
                winners.insert(b);
                if winners.len() == blocks.len() {
                    return Some(sum * draw);
                }
            }
        }
    }
    None
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let mut sections = input.split("\n\n");
    let numbers: Vec<i64> = sections
        .next()
        .ok_or(AocError::ParseError(
            "Could not parse bingo numbers".to_string(),
        ))?
        .split(',')
        .flat_map(|n| Some(n.parse::<i64>().ok()?))
        .collect();
    let blocks = sections
        .map(|b| {
            b.split('\n')
                .filter(|l| !l.is_empty())
                .map(|line| {
                    line.split(' ')
                        .flat_map(|n| Some(n.parse::<i64>().ok()?))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = play1(&numbers, &blocks);
    dbg!(&part1);

    let part2 = play2(&numbers, &blocks);
    dbg!(&part2);

    Ok(())
}

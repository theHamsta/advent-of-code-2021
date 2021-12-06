#![feature(bool_to_option)]

use anyhow::Context;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

fn fish(days_left: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    if let Some(&sum) = cache.get(&days_left) {
        sum
    } else {
        let mut sum = 1;
        let mut d = days_left.clone();
        while d > 0 {
            sum += fish(d.clone() - 9, cache);
            d -= 7;
        }
        cache.insert(days_left, sum.clone());
        sum
    }
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let input: Vec<i64> = input
        .lines()
        .next()
        .ok_or(AocError::ParseError("no first line".to_owned()))?
        .split(',')
        .flat_map(|n| n.parse())
        .collect();

    let mut cache = HashMap::new();
    let part1: i64 = input.iter().map(|f| fish(80 - f, &mut cache)).sum();
    dbg!(&part1);

    let part2: i64 = input.iter().map(|f| fish(256 - f, &mut cache)).sum();
    dbg!(&part2);

    Ok(())
}

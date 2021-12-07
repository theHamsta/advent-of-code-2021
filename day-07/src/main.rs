#![feature(bool_to_option)]

use anyhow::Context;

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

    let input: Vec<i64> = input
        .lines()
        .next()
        .ok_or(AocError::ParseError("no first line".to_owned()))?
        .split(',')
        .flat_map(|n| n.parse())
        .collect();

    let min = *input.iter().reduce(|a, b| a.min(b)).unwrap();
    let max = *input.iter().reduce(|a, b| a.max(b)).unwrap();

    let calc_fuel = |f: fn(i64, i64) -> i64| {
        (min..=max).min_by_key(|&pos| input.iter().map(|&p| f(p, pos)).sum::<i64>())
    };

    let part1 = calc_fuel(|p, pos| (p - pos).abs());
    dbg!(&part1);

    let part2 = calc_fuel(|p, pos| {
        let n = (pos - p).abs();
        n * (n + 1) / 2
    });
    dbg!(&part2);

    Ok(())
}

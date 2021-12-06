#![feature(bool_to_option)]

use anyhow::Context;
use num_bigint::{BigInt, ToBigInt};
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

fn fish(days_left: BigInt, cache: &mut HashMap<BigInt, BigInt>) -> BigInt {
    if let Some(sum) = cache.get(&days_left) {
        sum.clone()
    } else {
        let mut sum = 1.to_bigint().unwrap();
        let mut d = days_left.clone();
        while d > 0.to_bigint().unwrap() {
            sum += fish(d.clone() - 9.to_bigint().unwrap(), cache);
            d -= 7.to_bigint().unwrap();
        }
        cache.insert(days_left, sum.clone());
        sum
    }
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let input: Vec<BigInt> = input
        .lines()
        .next()
        .ok_or(AocError::ParseError("no first line".to_owned()))?
        .split(',')
        .flat_map(|n| Some(n.parse::<i64>().ok()?.to_bigint().unwrap()))
        .collect();

    let mut cache = HashMap::new();
    for day in 1..=18 {
        dbg!(&day);
        let part1: BigInt = input
            .iter()
            .map(|f| fish(day.to_bigint().unwrap() - f, &mut cache))
            .sum();
        dbg!(&part1);
    }

    let mut cache = HashMap::new();
    let part1: BigInt = input
        .iter()
        .map(|f| fish(80.to_bigint().unwrap() - f, &mut cache))
        .sum();
    dbg!(&part1);

    let part2: BigInt = input
        .iter()
        .map(|f| fish(256.to_bigint().unwrap() - f, &mut cache))
        .sum();
    dbg!(&part2);

    Ok(())
}

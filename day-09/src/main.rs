#![feature(bool_to_option)]

use std::{collections::HashMap, convert::identity};

use anyhow::Context;
use itertools::Itertools;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

fn offset(array: &Vec<Vec<i64>>, (x, y): (i64, i64), (dx, dy): (i64, i64)) -> Option<&i64> {
    array
        .get((y + dy) as usize)
        .iter()
        .flat_map(|v| v.get((x + dx) as usize))
        .next()
}

fn neighbors(array: &Vec<Vec<i64>>, pos: (i64, i64)) -> [Option<&i64>; 4] {
    [
        offset(array, pos, (1, 0)),
        offset(array, pos, (0, 1)),
        offset(array, pos, (-1, 0)),
        offset(array, pos, (0, -1)),
    ]
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let input: Vec<Vec<i64>> = input
        .lines()
        .map(|l| l.chars().flat_map(|c| format!("{}", c).parse()).collect())
        .collect();

    let mut sum = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let center = input[y][x];

            if neighbors(&input, (x.try_into()?, y.try_into()?))
                .iter()
                .flat_map(identity)
                .all(|&&n| center < n)
            {
                sum += 1 + center;
            }
        }
    }
    dbg!(&sum);

    let mut basin_scores = HashMap::new();

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let mut pos = (x.try_into()?, y.try_into()?);
            let mut center = input[y][x];
            if center == 9 {
                continue;
            }
            loop {
                center = input[pos.1 as usize][pos.0 as usize];
                let next = neighbors(&input, pos)
                    .iter()
                    .zip([(1, 0), (0, 1), (-1, 0), (0, -1)].iter())
                    .flat_map(|(val, pos)| val.map(|v| (v, pos)))
                    .filter(|(val, _)| center > **val)
                    .min_by_key(|(val, _pos)| *val);
                if let Some((_, &(dx, dy))) = next {
                    pos = (pos.0 + dx, pos.1 + dy);
                } else {
                    break;
                }
            }
            *basin_scores.entry(pos).or_insert(0) += 1;
        }
    }

    let part2: i64 = basin_scores.values().sorted().rev().take(3).product();
    dbg!(&part2);
    Ok(())
}

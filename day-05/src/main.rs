#![feature(bool_to_option)]

use std::collections::HashMap;

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

    let re = regex::Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    let input = re.captures_iter(&input).flat_map(|cap| {
        Some((
            (cap[1].parse::<i64>().ok()?, cap[2].parse::<i64>().ok()?),
            (cap[3].parse::<i64>().ok()?, cap[4].parse::<i64>().ok()?),
        ))
    });

    let mut diagram = HashMap::new();
    let mut diagram2 = HashMap::new();

    for i in input {
        match i {
            ((x1, y1), (x2, y2)) if x1 == x2 => {
                for y in y1.min(y2)..=y1.max(y2) {
                    *diagram.entry((x1, y)).or_insert(0i64) += 1;
                    *diagram2.entry((x1, y)).or_insert(0i64) += 1;
                }
            }
            ((x1, y1), (x2, y2)) if y1 == y2 => {
                for x in x1.min(x2)..=x1.max(x2) {
                    *diagram.entry((x, y1)).or_insert(0i64) += 1;
                    *diagram2.entry((x, y1)).or_insert(0i64) += 1;
                }
            }
            ((x1, y1), (x2, y2)) => {
                let mut x = x1.min(x2)..=x1.max(x2);
                let mut y = y1.min(y2)..=y1.max(y2);
                let mut x_rev = x.clone().rev();
                let mut y_rev = y.clone().rev();

                while let (Some(x), Some(y)) = (
                    if x1 < x2 { x.next() } else { x_rev.next() },
                    if y1 < y2 { y.next() } else { y_rev.next() },
                ) {
                    *diagram2.entry((x, y)).or_insert(0i64) += 1;
                }
            }
        }
    }

    let part1 = diagram.values().filter(|&&v| v >= 2).count();
    dbg!(&part1);

    let part2 = diagram2.values().filter(|&&v| v >= 2).count();
    dbg!(&part2);

    Ok(())
}

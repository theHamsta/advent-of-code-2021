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

#[derive(Debug)]
struct Input {
    from: (i64, i64),
    to: (i64, i64),
}

fn parse_pair(pair: &str) -> Option<(i64, i64)> {
    let mut it = pair.split(",");
    Some((it.next()?.parse().ok()?, it.next()?.parse().ok()?))
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let input = input.lines().filter(|l| !l.is_empty()).flat_map(|l| {
        let mut it = l.split(" -> ");
        Some(Input {
            from: parse_pair(it.next()?)?,
            to: parse_pair(it.next()?)?,
        })
    });

    let mut diagram = HashMap::new();
    let mut diagram2 = HashMap::new();

    for i in input {
        match i {
            Input {
                from: (x1, y1),
                to: (x2, y2),
            } if x1 == x2 => {
                for y in y1.min(y2)..=y1.max(y2) {
                    *diagram.entry((x1, y)).or_insert(0i64) += 1;
                    *diagram2.entry((x1, y)).or_insert(0i64) += 1;
                }
            }
            Input {
                from: (x1, y1),
                to: (x2, y2),
            } if y1 == y2 => {
                for x in x1.min(x2)..=x1.max(x2) {
                    *diagram.entry((x, y1)).or_insert(0i64) += 1;
                    *diagram2.entry((x, y1)).or_insert(0i64) += 1;
                }
            }
            Input {
                from: (x1, y1),
                to: (x2, y2),
            } => {
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

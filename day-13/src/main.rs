use std::collections::HashSet;

use anyhow::Context;
use regex::Regex;

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

    let re_dots = Regex::new(r"(\d+),(\d+)").unwrap();
    let re_folds = Regex::new(r"fold along (\w)=(\d*)").unwrap();

    let dots: HashSet<(i64, i64)> = re_dots
        .captures_iter(&input)
        .flat_map(|cap| Some((cap[1].parse().ok()?, cap[2].parse().ok()?)))
        .collect();

    let folds: Vec<(char, i64)> = re_folds
        .captures_iter(&input)
        .flat_map(|cap| Some((cap[1].chars().next()?, cap[2].parse().ok()?)))
        .collect();

    let mut dots_copy = dots.clone();
    for (axis, coordinate) in folds {
        dots_copy = dots_copy
            .iter()
            .map(|(x, y)| match axis {
                'x' => (
                    if *x < coordinate {
                        *x
                    } else {
                        2 * coordinate - *x
                    },
                    *y,
                ),
                'y' => (
                    *x,
                    if *y < coordinate {
                        *y
                    } else {
                        2 * coordinate - *y
                    },
                ),
                _ => unreachable!(),
            })
            .collect();
        dbg!(&dots_copy.len());
    }

    let max_x = dots_copy.iter().map(|(x, _)| x).max().unwrap();
    let max_y = dots_copy.iter().map(|(_, y)| y).max().unwrap();
    for y in 0..=*max_y {
        for x in 0..=*max_x {
            if dots_copy.contains(&(x, y)) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }

    Ok(())
}

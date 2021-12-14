#![feature(array_zip)]

use std::{cell::Cell, collections::HashMap, collections::HashSet, mem::swap, rc::Rc};

use anyhow::Context;
use itertools::Itertools;
use regex::Regex;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

fn expand_chars<'cache>(
    input: (char, char),
    rules: &HashMap<(char, char), char>,
    steps: usize,
    cache: &'cache mut HashMap<((char, char), usize), [u64; 26]>,
) -> &'cache [u64; 26] {
    let key = &(input, steps);
    if !cache.contains_key(key) {
        match rules.get(&input) {
            // expandable
            Some(&insertion) if steps > 0 => {
                let part1 = expand_chars((input.0, insertion), rules, steps - 1, cache).clone();
                let part2 = expand_chars((insertion, input.1), rules, steps - 1, cache).clone();
                cache.insert(*key, part1.zip(part2).map(|(p1, p2)| p1 + p2));
            }
            // in-expandable
            _ => {
                let mut array = [0u64; 26];
                array[input.1 as usize - 'A' as usize] = 1;
                cache.insert(*key, array);
            }
        }
    }
    &cache[key]
}

fn expand(
    input: &str,
    rules: &HashMap<(char, char), char>,
    steps: usize,
    cache: &mut HashMap<((char, char), usize), [u64; 26]>,
) -> Option<[u64; 26]> {
    let mut result = [0; 26];
    // Count first letter (will never be considers as second part of an unexpandable
    result[input.chars().next()? as usize - 'A' as usize] = 1;
    for (a, b) in input.chars().tuple_windows() {
        result = expand_chars((a, b), &rules, steps, cache)
            .zip(result)
            .map(|(p1, p2)| p1 + p2);
    }
    Some(result)
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;
    let template = input
        .lines()
        .next()
        .ok_or_else(|| AocError::ParseError("No first line".to_string()))?
        .to_string();

    let re = Regex::new(r"(\w)(\w) -> (\w)").unwrap();

    let rules: HashMap<_, _> = re
        .captures_iter(&input)
        .map(|cap| {
            (
                (
                    cap[1].chars().next().unwrap(),
                    cap[2].chars().next().unwrap(),
                ),
                cap[3].chars().next().unwrap(),
            )
        })
        .collect();

    let mut cache = HashMap::new();

    for (part, &steps) in [10, 40].iter().enumerate() {
        if let Some((min, max)) = expand(&template, &rules, steps, &mut cache).unwrap()
            .iter()
            .filter(|&&f| f != 0)
            .minmax()
            .into_option()
        {
            let solution = max - min;
            println!("part {part}: {solution}");
        }
    }

    Ok(())
}

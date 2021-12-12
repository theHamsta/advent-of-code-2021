use std::collections::{HashMap, HashSet};

use anyhow::Context;
use regex::Regex;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Cave {
    Big(String),
    Small(String),
}

fn depth_search<'a>(
    system: &'a HashMap<String, HashSet<Cave>>,
    cave_name: &'a str,
    visited: &im::HashMap<&'a Cave, u64>,
    second_visit_ok: bool,
) -> u64 {
    if cave_name == "end" {
        1
    } else {
        let successors = &system[cave_name];
        successors
            .iter()
            .map(|s| match (s, visited.get(s)) {
                (Cave::Small(name), None) => {
                    depth_search(system, name, &visited.update(&s, 1), second_visit_ok)
                }
                (Cave::Small(name), Some(&1)) if second_visit_ok => {
                    depth_search(system, name, &visited.update(&s, 2), false)
                }
                (Cave::Big(name), _) => depth_search(system, name, &visited, second_visit_ok),
                _ => 0,
            })
            .sum::<u64>()
    }
}


fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let re = Regex::new(r"(\w+)-(\w+)").unwrap();
    let re_small = Regex::new(r"^[a-z]*$").unwrap();

    let mut system = HashMap::new();
    re.captures_iter(&input).for_each(|cap| {
        let cave1 = if re_small.is_match(&cap[1]) {
            Cave::Small(cap[1].to_string())
        } else {
            Cave::Big(cap[1].to_string())
        };
        let cave2 = if re_small.is_match(&cap[2]) {
            Cave::Small(cap[2].to_string())
        } else {
            Cave::Big(cap[2].to_string())
        };

        system
            .entry(cap[1].to_string())
            .or_insert(HashSet::new())
            .insert(cave2);
        system
            .entry(cap[2].to_string())
            .or_insert(HashSet::new())
            .insert(cave1);
    });

    let part1 = depth_search(
        &system,
        "start",
        &im::HashMap::unit(&Cave::Small("start".into()), 2),
        false,
    );
    dbg!(&part1);

    let part2 = depth_search(
        &system,
        "start",
        &im::HashMap::unit(&Cave::Small("start".into()), 2),
        true,
    );
    dbg!(&part2);

    Ok(())
}

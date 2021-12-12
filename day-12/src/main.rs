use std::collections::{HashMap, HashSet};

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

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Cave<'a> {
    Big(&'a str),
    Small(&'a str),
}

fn depth_search<'a, 'cache>(
    system: &'a HashMap<String, HashSet<Cave<'a>>>,
    cave_name: &'a str,
    visited: &'cache im::HashMap<&'a Cave<'a>, u64>,
    second_visit_ok: bool,
    // With cache 243ms else ~1.9s
    cache: &'cache mut HashMap<(&'a str, im::HashMap<&'a Cave<'a>, u64>, bool), u64>,
) -> u64 {
    if cave_name == "end" {
        1
    } else if let Some(&cached_result) = cache.get(&(cave_name, visited.clone(), second_visit_ok)) {
        cached_result
    } else {
        let successors = &system[cave_name];
        let rtn = successors
            .iter()
            .map(|s| match (s, visited.get(s)) {
                (Cave::Small(name), None) => {
                    depth_search(system, name, &visited.update(&s, 1), second_visit_ok, cache)
                }
                (Cave::Small(name), Some(&1)) if second_visit_ok => {
                    depth_search(system, name, &visited.update(&s, 2), false, cache)
                }
                (Cave::Big(name), _) => {
                    depth_search(system, name, &visited, second_visit_ok, cache)
                }
                _ => 0,
            })
            .sum::<u64>();
        cache.insert((cave_name, visited.clone(), second_visit_ok), rtn);
        rtn
    }
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let re = Regex::new(r"(\w+)-(\w+)").unwrap();
    let re_small = Regex::new(r"^[a-z]*$").unwrap();

    let mut system = HashMap::new();
    let captures = re.captures_iter(&input).collect_vec();
    captures.iter().for_each(|cap| {
        let cave1 = if re_small.is_match(&cap[1]) {
            Cave::Small(&cap[1])
        } else {
            Cave::Big(&cap[1])
        };
        let cave2 = if re_small.is_match(&cap[2]) {
            Cave::Small(&cap[2])
        } else {
            Cave::Big(&cap[2])
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

    let start_node = Cave::Small("start");
    let mut cache = HashMap::new();
    let part1 = depth_search(
        &system,
        "start",
        &im::HashMap::unit(&start_node, 2),
        false,
        &mut cache,
    );
    dbg!(&part1);

    let part2 = depth_search(
        &system,
        "start",
        &im::HashMap::unit(&start_node, 2),
        true,
        &mut cache,
    );
    dbg!(&part2);

    Ok(())
}

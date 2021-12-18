use anyhow::Context;
use itertools::Itertools;
use serde_json::{json, Value};

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

fn snail_fish_reduce(value: &mut Value) -> bool {
    snail_fish_explode(value, 0, None, None) || snail_fish_split(value)
}

fn snail_fish_split(value: &mut Value) -> bool {
    match value {
        Value::Number(n) if n.as_u64().unwrap() >= 10 => {
            *value = json!([n.as_u64().unwrap() / 2, (n.as_u64().unwrap() + 1) / 2]);
            true
        }
        Value::Array(children) => children.iter_mut().any(snail_fish_split),
        _ => false,
    }
}

fn add_nested(a: Option<&mut Value>, b: u64, index: usize) {
    match a {
        Some(Value::Number(n)) => {
            *n = (n.as_u64().unwrap() + b).into();
        }
        Some(Value::Array(children)) => {
            add_nested(Some(&mut children[index]), b, index);
        }
        _ => (),
    }
}

fn snail_fish_explode(
    value: &mut Value,
    nesting_level: u64,
    left: Option<&mut Value>,
    right: Option<&mut Value>,
) -> bool {
    if let Value::Array(children) = value {
        if nesting_level >= 3 {
            if let Some((idx, to_explode)) = children
                .iter()
                .enumerate()
                .flat_map(|(idx, c)| Some((idx, c.as_array()?)))
                .next()
            {
                if let [Value::Number(a), Value::Number(b)] = &to_explode[..] {
                    let a = a.clone();
                    let b = b.clone();
                    if idx == 0 {
                        *value = json!([0u64, children[1]]);

                        add_nested(left, a.as_u64().unwrap(), 1);
                        add_nested(Some(&mut value[1]), b.as_u64().unwrap(), 0);
                        return true;
                    } else if idx == 1 {
                        *value = json!([children[0], 0u64]);

                        add_nested(Some(&mut value[0]), a.as_u64().unwrap(), 1);
                        add_nested(right, b.as_u64().unwrap(), 0);
                        return true;
                    } else {
                        unreachable!();
                    }
                } else {
                    unreachable!();
                }
            }
        }
        let (l, r) = children.split_at_mut(1);
        return snail_fish_explode(&mut l[0], nesting_level + 1, left, Some(&mut r[0]))
            || snail_fish_explode(&mut r[0], nesting_level + 1, Some(&mut l[0]), right);
    } else {
        false
    }
}

fn snail_fish_add(a: Value, b: Value) -> Value {
    let mut current = json!([a, b]);
    while snail_fish_reduce(&mut current) {}
    current
}

fn magnitude(value: &Value) -> u64 {
    match value {
        Value::Number(n) => n.as_u64().unwrap(),
        Value::Array(children) => 3 * magnitude(&children[0]) + 2 * magnitude(&children[1]),
        _ => unreachable!(),
    }
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let parsed = input.lines().flat_map(serde_json::from_str).collect_vec();
    let part1 = parsed
        .iter()
        .cloned()
        .reduce(snail_fish_add)
        .ok_or_else(|| AocError::ParseError("No input not parsed successfully".to_string()))?;
    let part1 = magnitude(&part1);
    dbg!(part1);

    let part2 = parsed
        .iter()
        .permutations(2)
        .map(|vec| magnitude(&snail_fish_add(vec[0].clone(), vec[1].clone())))
        .max();
    dbg!(part2);

    Ok(())
}

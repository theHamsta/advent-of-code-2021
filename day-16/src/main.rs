use std::str::Chars;

use anyhow::Context;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Max,
    Min,
    GreaterThan,
    LessThan,
    Eq,
}

#[derive(Debug)]
enum PackageType {
    Literal(u64),
    Operator(Op, Vec<Package>),
}

#[derive(Debug)]
struct Package {
    version: u64,
    package_type: PackageType,
}

fn parse_literal(it: &mut Chars, buffer: &mut String, consumed: &mut usize) -> u64 {
    buffer.clear();
    let mut keep_going = true;
    while keep_going {
        keep_going = it.next().unwrap() == '1';
        for _ in 0..4 {
            buffer.push(it.next().unwrap());
        }
        *consumed += 5;
    }
    u64::from_str_radix(&buffer, 2).unwrap()
}

fn parse_operator(it: &mut Chars, buffer: &mut String, consumed: &mut usize) -> Vec<Package> {
    buffer.clear();
    let length = if it.next().unwrap() == '0' { 15 } else { 11 };
    for _ in 0..length {
        buffer.push(it.next().unwrap());
    }

    *consumed += length + 1;

    let sub_packages = usize::from_str_radix(&buffer, 2).unwrap();
    let mut packages = Vec::new();
    if length == 11 {
        for _ in 0..sub_packages {
            packages.push(parse_package(it, buffer, consumed));
        }
    } else {
        let before = *consumed;
        while *consumed < before + sub_packages {
            packages.push(parse_package(it, buffer, consumed));
        }
    }
    packages
}

fn parse_package(it: &mut Chars, buffer: &mut String, consumed: &mut usize) -> Package {
    buffer.clear();
    for _ in 0..3 {
        buffer.push(it.next().unwrap());
    }
    *consumed += 3;
    let version = u64::from_str_radix(&buffer, 2).unwrap();
    buffer.clear();
    for _ in 0..3 {
        buffer.push(it.next().unwrap());
    }
    *consumed += 3;
    let type_id = u64::from_str_radix(&buffer, 2).unwrap();
    Package {
        version,
        package_type: match type_id {
            0 => PackageType::Operator(Op::Sum, parse_operator(it, buffer, consumed)),
            1 => PackageType::Operator(Op::Product, parse_operator(it, buffer, consumed)),
            2 => PackageType::Operator(Op::Min, parse_operator(it, buffer, consumed)),
            3 => PackageType::Operator(Op::Max, parse_operator(it, buffer, consumed)),
            4 => PackageType::Literal(parse_literal(it, buffer, consumed)),
            5 => PackageType::Operator(Op::GreaterThan, parse_operator(it, buffer, consumed)),
            6 => PackageType::Operator(Op::LessThan, parse_operator(it, buffer, consumed)),
            7 => PackageType::Operator(Op::Eq, parse_operator(it, buffer, consumed)),
            _ => unreachable!(),
        },
    }
}

fn version_sum(package: &Package) -> u64 {
    package.version
        + match &package.package_type {
            PackageType::Literal(_) => 0,
            PackageType::Operator(_, packages) => packages.iter().map(|p| version_sum(&p)).sum(),
        }
}

fn evaluate(package: &Package) -> u64 {
    match &package.package_type {
        PackageType::Literal(number) => *number,
        PackageType::Operator(o, packages) => {
            let mut evaluated = packages.iter().map(|p| evaluate(p));
            match o {
                Op::Sum => evaluated.sum(),
                Op::Product => evaluated.product(),
                Op::Min => evaluated.min().unwrap(),
                Op::Max => evaluated.max().unwrap(),
                Op::GreaterThan => {
                    let first = evaluated.next().unwrap();
                    evaluated.all(|p| first > p) as u64
                }
                Op::LessThan => {
                    let first = evaluated.next().unwrap();
                    evaluated.all(|p| first < p) as u64
                }
                Op::Eq => {
                    let first = evaluated.next().unwrap();
                    evaluated.all(|p| first == p) as u64
                }
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;
    let mut input_binary = String::new();

    input
        .chars()
        .flat_map(|c| u32::from_str_radix(&format!("{}", c), 16))
        .for_each(|n| input_binary.push_str(&format!("{:04b}", n)));

    let mut consumed = 0usize;
    let mut it = input_binary.chars();
    let mut buffer = String::new();
    let parsed = parse_package(&mut it, &mut buffer, &mut consumed);
    let part1 = version_sum(&parsed);

    dbg!(part1);

    let part2 = evaluate(&parsed);
    dbg!(part2);

    Ok(())
}

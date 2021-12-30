use std::io::prelude::*;
use std::{collections::HashMap, fs::File, rc::Rc};

use anyhow::Context;

mod alu;
mod generated;
mod instruction;
mod op;

use alu::Alu;
use indicatif::ProgressIterator;
use instruction::Instruction;
use itertools::Itertools;
use maplit::hashmap;

use crate::instruction::parse_instructions;

#[derive(thiserror::Error, Debug)]
pub(crate) enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
    #[error("not input left for `inp` instruction")]
    NoAluInputLeft,
    #[error("invalid instruction {0:?}")]
    InvalidInstruction(Instruction),
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let instructions = parse_instructions(&input)?;

    if let Some(last) = std::env::args().last() {
        if last == "--dot" {
            let expression = Rc::clone(&Alu::symbolic_execution(&instructions)?[&'z']);
            println!("{}", expression.to_dot());
            return Ok(());
        }
    }
    let subprograms = instructions
        .split(|instr| matches!(instr, &Instruction::Input(_)))
        .filter(|p| !p.is_empty())
        .collect_vec();
    assert_eq!(subprograms.len(), 14);

    //let mut file = File::create("generated.rs")?;
    //writeln!(file, "fn prog(prog_idx: usize, w: i64, z: i64) -> i64 {{");
    //writeln!(file, "match prog_idx {{");
    //for (idx, s) in subprograms.iter().enumerate() {
    //let mut prog = vec![Instruction::Input('w'), Instruction::Input('z')];
    //prog.extend_from_slice(s);
    //writeln!(file, "{idx} =>");
    //writeln!(file, "{},", Alu::symbolic_execution(&prog)?[&'z']);
    //}
    //writeln!(file, "}}");
    //writeln!(file, "}}");

    let just_zero = hashmap! { 0i64 => vec![]};
    let mut partial_solutions = Vec::new();
    subprograms
        .iter()
        .progress()
        .enumerate()
        .for_each(|(idx, &p)| {
            let mut partial_solution = HashMap::new();
            let bound = 26i64.pow(14 - idx as u32);
            (1..=9)
                .cartesian_product(
                    if partial_solutions.is_empty() {
                        &just_zero
                    } else {
                        &partial_solutions[partial_solutions.len() - 1]
                    }
                    .keys(),
                )
                .progress_count(
                    if partial_solutions.is_empty() {
                        &just_zero
                    } else {
                        &partial_solutions[partial_solutions.len() - 1]
                    }
                    .len() as u64
                        * 10,
                )
                .for_each(|(input, &z)| {
                    // bound found on solution thread after solving this without it
                    if z <= bound {
                        let interpreted = true;
                        let result = if interpreted {
                            let mut alu = Alu::default();
                            *alu.register_mut('z') = z;
                            *alu.register_mut('w') = input;
                            alu.run(p, &[input]).unwrap();
                            alu.register('z')
                        } else {
                            generated::prog(idx, input, z)
                        };
                        partial_solution
                            .entry(result)
                            .or_insert_with(|| Vec::new())
                            .push((input, z));
                    }
                });
            partial_solutions.push(partial_solution);
        });

    let part1 = solve_max(&partial_solutions, 0);
    dbg!(&part1);
    let part2 = solve_min(&partial_solutions, 0);
    dbg!(&part2);

    Ok(())
}

fn solve_max(partial_solutions: &[HashMap<i64, Vec<(i64, i64)>>], target: i64) -> Option<i64> {
    match partial_solutions {
        [] => Some(0),
        [rest @ .., last] => {
            let potential_solutions = &last.get(&target)?;
            potential_solutions
                .into_iter()
                .flat_map(|&(input, previous)| Some(input + 10 * solve_max(rest, previous)?))
                .max()
        }
    }
}

fn solve_min(partial_solutions: &[HashMap<i64, Vec<(i64, i64)>>], target: i64) -> Option<i64> {
    match partial_solutions {
        [] => Some(0),
        [rest @ .., last] => {
            let potential_solutions = &last.get(&target)?;
            potential_solutions
                .into_iter()
                .flat_map(|&(input, previous)| Some(input + 10 * solve_min(rest, previous)?))
                .min()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_binary() {
        let program = include_str!("../example");
        let instructions = parse_instructions(program).unwrap();
        let input = vec![3];

        let mut alu = Alu::default();
        alu.run(&instructions, &input).unwrap();

        assert_eq!(alu.registers()[&'w'], 0);
        assert_eq!(alu.registers()[&'x'], 0);
        assert_eq!(alu.registers()[&'y'], 1);
        assert_eq!(alu.registers()[&'z'], 1);

        let input = vec![8];
        alu.reset();
        alu.run(&instructions, &input).unwrap();
        assert_eq!(alu.registers()[&'w'], 1);
        assert_eq!(alu.registers()[&'x'], 0);
        assert_eq!(alu.registers()[&'y'], 0);
        assert_eq!(alu.registers()[&'z'], 0);
    }

    #[test]
    fn convert_binary_symbolic() {
        let program = include_str!("../example");
        let instructions = parse_instructions(program).unwrap();
        let expression = Alu::symbolic_execution(&instructions).unwrap();
        assert_eq!(
            format!("{}", &expression[&'w']),
            "((((input0 / 2) / 2) / 2) % 2)"
        );
        assert_eq!(format!("{}", &expression[&'x']), "(((input0 / 2) / 2) % 2)");
        assert_eq!(format!("{}", &expression[&'y']), "((input0 / 2) % 2)");
        assert_eq!(format!("{}", &expression[&'z']), "(input0 % 2)");
    }

    #[test]
    fn example_to_dot() {
        let program = include_str!("../example");
        let instructions = parse_instructions(program).unwrap();
        let expression = Alu::symbolic_execution(&instructions).unwrap();
        println!("{}", expression[&'w'].to_dot());
    }
}

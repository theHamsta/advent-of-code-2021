#![allow(dead_code)]
use std::collections::HashMap;

use anyhow::Context;

mod alu;
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

    let subprograms = instructions
        .split(|instr| matches!(instr, &Instruction::Input(_)))
        .filter(|p| !p.is_empty())
        .collect_vec();
    assert_eq!(subprograms.len(), 14);

    let just_zero = hashmap! { 0i64 => vec![]};
    let mut partial_solutions = Vec::new();
    subprograms.iter().progress().for_each(|p| {
        let mut partial_solution = HashMap::new();
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
                let mut alu = Alu::default();
                *alu.register_mut('z') = z;
                *alu.register_mut('w') = input;
                alu.run(p, &[input]).unwrap();
                partial_solution
                    .entry(alu.register('z'))
                    .or_insert_with(|| Vec::new())
                    .push((input, z));
            });
        partial_solutions.push(partial_solution);
    });

    let part1 = solve_max(&partial_solutions, 0);
    dbg!(&part1);
    let part2 = solve_min(&partial_solutions, 0);
    dbg!(&part2);

    //let expression = Rc::clone(&Alu::symbolic_execution(&instructions)?[&'z']);
    //println!("{}", expression.to_dot());

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

    //#[test]
    //fn check_program() {
    //let program = include_str!("../input");
    //let instructions = parse(program).unwrap();
    //let mut alu = Alu::default();
    //const INPUT: [i64; 14] = [13, 15, 14, -4, -10, 11, 9, -12, 10, -11, 12, -1, 0, 11];
    //alu.run(&instructions, &INPUT).unwrap();

    //assert_eq!(alu.registers()[&'z'], mysterious_program(&INPUT));
    //}
}

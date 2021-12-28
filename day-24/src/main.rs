use std::collections::{HashMap, HashSet};
use std::convert::identity;
use std::fmt::Display;
use std::rc::Rc;

use anyhow::Context;

use combine::parser::char::{digit, letter, newline, spaces};
use combine::parser::range::range;
use combine::parser::token::token;
use combine::stream::{self, position};
use combine::{
    choice, many1, optional, sep_by, EasyParser, ParseError, Parser, RangeStreamOnce, StreamOnce,
};
use maplit::hashmap;

#[derive(thiserror::Error, Debug)]
enum AocError {
    #[error("No input file provided")]
    NoInputFile,
    #[error("Failed to parse: {0}")]
    ParseError(String),
    #[error("not input left for `inp` instruction")]
    NoAluInputLeft,
    #[error("invalid instruction {0:?}")]
    InvalidInstruction(Instruction),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Operand {
    Literal(i64),
    Register(char),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Instruction {
    Input(char),
    Mod(Operand, Operand),
    Div(Operand, Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Eql(Operand, Operand),
}

fn make_parser<'input, Input>() -> impl Parser<Input, Output = Vec<Instruction>> + 'input
where
    Input: StreamOnce<Range = &'input str, Token = char>
        + stream::ResetStream
        + stream::Positioned
        + RangeStreamOnce
        + 'input,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let operand = || {
        choice((
            letter().map(|c| Operand::Register(c)),
            (optional(token('-')), many1(digit())).map(|(sign, digits): (Option<char>, String)| {
                Operand::Literal(
                    format!("{}{}", if sign.is_some() { "-" } else { "" }, digits)
                        .parse::<i64>()
                        .unwrap(),
                )
            }),
        ))
    };

    let instruction = choice((
        (range("inp"), spaces(), letter()).map(|(_, _, l)| Instruction::Input(l)),
        (range("add"), spaces(), operand(), spaces(), operand())
            .map(|(_, _, o1, _, o2)| Instruction::Add(o1, o2)),
        (range("mul"), spaces(), operand(), spaces(), operand())
            .map(|(_, _, o1, _, o2)| Instruction::Mul(o1, o2)),
        (range("div"), spaces(), operand(), spaces(), operand())
            .map(|(_, _, o1, _, o2)| Instruction::Div(o1, o2)),
        (range("mod"), spaces(), operand(), spaces(), operand())
            .map(|(_, _, o1, _, o2)| Instruction::Mod(o1, o2)),
        (range("eql"), spaces(), operand(), spaces(), operand())
            .map(|(_, _, o1, _, o2)| Instruction::Eql(o1, o2)),
    ));
    Parser::map(
        sep_by(optional(instruction), newline()),
        |f: Vec<Option<Instruction>>| f.into_iter().flat_map(identity).collect(),
    )
}

fn parse(input: &str) -> anyhow::Result<Vec<Instruction>> {
    let (result, _) = make_parser()
        .easy_parse(position::Stream::new(input))
        .map_err(|e| AocError::ParseError(format!("{}", e)))?;
    Ok(result)
}

#[derive(Eq, PartialEq, Debug)]
enum Op {
    Input(u8),
    Value(i64),
    Mod(Rc<Op>, Rc<Op>),
    Div(Rc<Op>, Rc<Op>),
    Add(Rc<Op>, Rc<Op>),
    Mul(Rc<Op>, Rc<Op>),
    Eql(Rc<Op>, Rc<Op>),
    Neql(Rc<Op>, Rc<Op>),
    If(Rc<Op>, Rc<Op>),
}

impl Op {
    fn type_string(&self) -> String {
        match self {
            Op::Input(i) => format!("input{i}"),
            Op::Value(value) => format!("{value}"),
            Op::Mod(_, _) => format!("%"),
            Op::Div(_, _) => format!("/"),
            Op::Add(_, _) => format!("+"),
            Op::Mul(_, _) => format!("*"),
            Op::Eql(_, _) => format!("="),
            Op::Neql(_, _) => format!("!="),
            Op::If(_, _) => format!("if"),
        }
    }
    fn to_dot(&self) -> String {
        let mut nodes = HashSet::new();
        let mut edges = Vec::new();
        let mut stack = Vec::new();
        stack.push(self);
        while let Some(node) = stack.pop() {
            let label = node.type_string();
            if nodes.insert(format!("n{:p}[label=\"{label}\"]", node)) {
                match &*node {
                    Op::Mod(a, b)
                    | Op::Add(a, b)
                    | Op::Mul(a, b)
                    | Op::Eql(a, b)
                    | Op::Neql(a, b) => {
                        edges.push(format!("n{:p} -> n{:p}", *a, node));
                        stack.push(&**a);
                        edges.push(format!("n{:p} -> n{:p}", *b, node));
                        stack.push(&**b);
                    }
                    Op::If(a, b) => {
                        edges.push(format!("n{:p} -> n{:p} [label=\"condition\"]", &**a, node));
                        stack.push(&**a);
                        edges.push(format!("n{:p} -> n{:p}", *b, node));
                        stack.push(&**b);
                    }
                    Op::Div(a, b) => {
                        edges.push(format!("n{:p} -> n{:p}", *a, node));
                        stack.push(&**a);
                        edges.push(format!("n{:p} -> n{:p} [label=\"divide by\"]", &**b, node));
                        stack.push(&**b);
                    }
                    _ => (),
                }
            }
        }
        let mut rtn = String::new();
        rtn.push_str("digraph N {\n");
        for n in nodes {
            rtn.push_str(&n);
            rtn.push_str("\n");
        }
        for e in edges {
            rtn.push_str(&e);
            rtn.push_str("\n");
        }
        rtn.push_str("}\n");
        rtn
    }
}

#[derive(Default, Debug)]
struct Alu {
    registers: HashMap<char, i64>,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Input(number) => f.write_fmt(format_args!("input{}", number)),
            Op::Value(value) => f.write_fmt(format_args!("{}", value)),
            Op::Mod(a, b) => f.write_fmt(format_args!("({a} % {b})")),
            Op::Div(a, b) => f.write_fmt(format_args!("({a} / {b})")),
            Op::Add(a, b) => f.write_fmt(format_args!("({a} + {b})")),
            Op::Mul(a, b) => f.write_fmt(format_args!("({a} * {b})")),
            Op::Eql(a, b) => f.write_fmt(format_args!("(({a} == {b}) as i64)")),
            Op::If(a, b) => f.write_fmt(format_args!("(if {a} {{ {b} }} else {{ 0 }})")),
            Op::Neql(a, b) => f.write_fmt(format_args!("(({a} != {b}) as i64)")),
        }
    }
}

impl Alu {
    fn run(&mut self, instructions: &[Instruction], input: &[i64]) -> anyhow::Result<()> {
        let mut input_counter = 0;
        for i in instructions {
            match i {
                &Instruction::Input(reg) => {
                    *self.registers.entry(reg).or_default() =
                        *input.get(input_counter).ok_or(AocError::NoAluInputLeft)?;
                    input_counter += 1;
                }
                &Instruction::Mul(Operand::Register(reg), op) => {
                    *self.registers.entry(reg).or_default() *= self.get_value(op);
                }
                &Instruction::Add(Operand::Register(reg), op) => {
                    *self.registers.entry(reg).or_default() += self.get_value(op);
                }
                &Instruction::Mod(Operand::Register(reg), op) => {
                    *self.registers.entry(reg).or_default() %= self.get_value(op);
                }
                &Instruction::Div(Operand::Register(reg), op) => {
                    *self.registers.entry(reg).or_default() /= self.get_value(op);
                }
                &Instruction::Eql(Operand::Register(reg), op) => {
                    let op = self.get_value(op);
                    let reg = self.registers.entry(reg).or_default();
                    *reg = (*reg == op) as i64;
                }
                _ => return Err(AocError::InvalidInstruction(i.clone()).into()),
            }
        }
        Ok(())
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn get_value(&mut self, o: Operand) -> i64 {
        match o {
            Operand::Register(reg) => *self.registers.entry(reg).or_default(),
            Operand::Literal(value) => value,
        }
    }

    fn symbolic_execution(
        instructions: &Vec<Instruction>,
    ) -> anyhow::Result<HashMap<char, Rc<Op>>> {
        let mut input_counter = 0;

        let mut registers = hashmap!(
            'w' => Rc::new(Op::Value(0)),
            'x' => Rc::new(Op::Value(0)),
            'y' => Rc::new(Op::Value(0)),
            'z' => Rc::new(Op::Value(0))
        );

        for i in instructions {
            match i {
                Instruction::Input(dest) => {
                    registers.insert(*dest, Rc::new(Op::Input(input_counter)));
                    input_counter += 1;
                }
                Instruction::Mod(Operand::Register(reg), op) => {
                    let lhs = Rc::clone(&registers[reg]);
                    let rhs = match op {
                        Operand::Literal(val) => Rc::new(Op::Value(*val)),
                        Operand::Register(char) => Rc::clone(&registers[&char]),
                    };
                    registers.insert(
                        *reg,
                        Rc::new(match (&*lhs, &*rhs) {
                            (Op::Value(0), _) => Op::Value(0),
                            _ => Op::Mod(lhs, rhs),
                        }),
                    );
                }
                Instruction::Div(Operand::Register(reg), op) => {
                    let lhs = Rc::clone(&registers[reg]);
                    let rhs = match op {
                        Operand::Literal(val) => Rc::new(Op::Value(*val)),
                        Operand::Register(char) => Rc::clone(&registers[&char]),
                    };
                    registers.insert(
                        *reg,
                        match (&*lhs, &*rhs) {
                            (Op::Value(0), _) => Rc::new(Op::Value(0)),
                            (_, Op::Value(1)) => lhs,
                            _ => Rc::new(Op::Div(lhs, rhs)),
                        },
                    );
                }
                Instruction::Add(Operand::Register(reg), op) => {
                    let lhs = Rc::clone(&registers[reg]);
                    let rhs = match op {
                        Operand::Literal(val) => Rc::new(Op::Value(*val)),
                        Operand::Register(char) => Rc::clone(&registers[&char]),
                    };
                    registers.insert(
                        *reg,
                        match (&*lhs, &*rhs) {
                            (Op::Value(0), _) => rhs,
                            (_, Op::Value(0)) => lhs,
                            _ => Rc::new(Op::Add(lhs, rhs)),
                        },
                    );
                }
                Instruction::Mul(Operand::Register(reg), op) => {
                    let lhs = Rc::clone(&registers[reg]);
                    let rhs = match op {
                        Operand::Literal(val) => Rc::new(Op::Value(*val)),
                        Operand::Register(char) => Rc::clone(&registers[&char]),
                    };
                    registers.insert(
                        *reg,
                        match (&*lhs, &*rhs) {
                            (Op::Value(1), _) => Rc::clone(&rhs),
                            (_, Op::Value(1)) => Rc::clone(&lhs),
                            (Op::Value(0), _) => Rc::new(Op::Value(0)),
                            (_, Op::Value(0)) => Rc::new(Op::Value(0)),
                            (Op::Eql(..), _) => Rc::new(Op::If(lhs, rhs)),
                            (_, Op::Eql(..)) => Rc::new(Op::If(rhs, lhs)),
                            (Op::Neql(..), _) => Rc::new(Op::If(lhs, rhs)),
                            (_, Op::Neql(..)) => Rc::new(Op::If(rhs, lhs)),
                            _ => Rc::new(Op::Mul(lhs, rhs)),
                        },
                    );
                }
                Instruction::Eql(Operand::Register(reg), op) => {
                    let lhs = Rc::clone(&registers[reg]);
                    let rhs = match op {
                        Operand::Literal(val) => Rc::new(Op::Value(*val)),
                        Operand::Register(char) => Rc::clone(&registers[&char]),
                    };
                    registers.insert(
                        *reg,
                        Rc::new(match (&*lhs, &*rhs) {
                            (Op::Value(a), Op::Value(b)) => Op::Value((a == b) as i64),
                            (Op::Value(0 | 10..), Op::Input(_)) => Op::Value(0),
                            (Op::Input(_), Op::Value(0 | 10..)) => Op::Value(0),
                            (Op::Eql(a, b), Op::Value(0)) => Op::Neql(Rc::clone(a), Rc::clone(b)),
                            (Op::Value(0), Op::Eql(a, b)) => Op::Neql(Rc::clone(a), Rc::clone(b)),
                            (Op::Eql(a, b), Op::Value(1)) => Op::Eql(Rc::clone(a), Rc::clone(b)),
                            (Op::Value(1), Op::Eql(a, b)) => Op::Eql(Rc::clone(a), Rc::clone(b)),
                            (Op::Eql(..), Op::Value(2..)) => Op::Value(0),
                            (Op::Value(2..), Op::Eql(..)) => Op::Value(0),
                            _ => Op::Eql(lhs, rhs),
                        }),
                    );
                }
                _ => return Err(AocError::InvalidInstruction(i.clone()).into()),
            }
        }
        Ok(registers)
    }
}

const X: [i64; 14] = [11, 14, 13, -4, -11, 10, 9, -12, 10, -11, 12, -1, 0, 11];
const Y: [i64; 14] = [3, 7, 1, 6, 14, 7, 9, 9, 6, 4, 0, 7, 12, 1];
const DIVIDE: [bool; 14] = [
    false, false, true, false, false, false, true, true, false, true, false, true, true, true,
];

fn mysterious_program(input: &[i64]) -> i64 {
    let mut a = 0_i64;
    for i in 0..14 {
        if DIVIDE[i] {
            a /= 26;
        }
        let rest = a % 26 + X[i];
        if input[i] != rest {
            a = 26 * a + (input[i] + Y[i]);
        }
    }
    a
}

/*fn reverse_execution<const N: usize>(decisions: [bool; N]) -> [i64; N] {*/
/*let mut a = 0i64;*/
/*let mut input = [0i64; N];*/
/*for i in (0..N).rev() {*/
/*let rest = a % 26 + X[i];*/
/*if decisions[i] {*/
/*input[i] = rest;*/
/*a *= 26;*/
/*} else {*/
/*}*/
/*if DIVIDE[i] {*/
/*a /= 26;*/
/*}*/
/*}*/
/*a*/
/*}*/

fn solve(expr: &Rc<Op>, target_value: i64) -> Vec<HashMap<*const Op, i64>> {
    let mut visited = HashSet::new();
    let mut inputs = Vec::new();
    let mut input_ranges = Vec::new();
    let mut stack = Vec::new();
    stack.push(&**expr);
    while let Some(node) = stack.pop() {
        if visited.insert(node as *const Op) {
            match node {
                Op::Input(_) => {
                    inputs.push(node);
                    input_ranges.push(1..=9i64);
                }
                Op::Add(a, b)
                    if node as *const Op != &**expr as *const Op
                        && ((matches!(**a, Op::Mul(..)) && matches!(**b, Op::If(..)))
                            || (matches!(**b, Op::Mul(..)) && matches!(**a, Op::If(..)))) =>
                {
                    inputs.push(node);
                    input_ranges.push(1..=(26i64 * 26));
                }
                Op::Mod(a, b)
                | Op::Add(a, b)
                | Op::Mul(a, b)
                | Op::Eql(a, b)
                | Op::If(a, b)
                | Op::Div(a, b)
                | Op::Neql(a, b) => {
                    stack.push(&**a);
                    stack.push(&**b);
                }
                _ => (),
            }
        }
    }
    let solutions = Vec::new();
    solutions
}

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).ok_or(AocError::NoInputFile)?;
    let input = std::fs::read_to_string(file).context("Failed to read input file")?;

    let instructions = parse(&input)?;

    let expression = Rc::clone(&Alu::symbolic_execution(&instructions)?[&'z']);
    //println!("{expression}");

    println!("{}", expression.to_dot());
    assert_eq!(0, mysterious_program(&X));

    dbg!(solve(&expression, 0));

    //let a = 10u64.pow(14);
    //let part1 = repeat_n((1..=9).rev(), 14)
    //.multi_cartesian_product()
    //.enumerate()
    //.find(|(i, vec)| {
    //if i % 1_000_000 == 0 {
    //println!("{}", *i as f32 * 100.0 / a as f32);
    //}
    //mysterious_program(vec) == 0
    //});
    //dbg!(&part1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_binary() {
        let program = include_str!("../example");
        let instructions = parse(program).unwrap();
        let input = vec![3];

        let mut alu = Alu::default();
        alu.run(&instructions, &input).unwrap();

        assert_eq!(alu.registers[&'w'], 0);
        assert_eq!(alu.registers[&'x'], 0);
        assert_eq!(alu.registers[&'y'], 1);
        assert_eq!(alu.registers[&'z'], 1);

        let input = vec![8];
        alu.reset();
        alu.run(&instructions, &input).unwrap();
        assert_eq!(alu.registers[&'w'], 1);
        assert_eq!(alu.registers[&'x'], 0);
        assert_eq!(alu.registers[&'y'], 0);
        assert_eq!(alu.registers[&'z'], 0);
    }

    #[test]
    fn convert_binary_symbolic() {
        let program = include_str!("../example");
        let instructions = parse(program).unwrap();
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
        let instructions = parse(program).unwrap();
        let expression = Alu::symbolic_execution(&instructions).unwrap();
        println!("{}", expression[&'w'].to_dot());
    }

    #[test]
    fn check_program() {
        let program = include_str!("../input");
        let instructions = parse(program).unwrap();
        let mut alu = Alu::default();
        const INPUT: [i64; 14] = [13, 15, 14, -4, -10, 11, 9, -12, 10, -11, 12, -1, 0, 11];
        alu.run(&instructions, &INPUT).unwrap();

        assert_eq!(alu.registers[&'z'], mysterious_program(&INPUT));
    }
}

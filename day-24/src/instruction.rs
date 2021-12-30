use combine::parser::char::{digit, letter, spaces};
use combine::parser::range::range;
use combine::parser::token::token;
use combine::stream::{self, position};
use combine::{
    choice, many1, optional, sep_by, EasyParser, ParseError, Parser, RangeStreamOnce, StreamOnce,
};

use crate::AocError;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Operand {
    Literal(i64),
    Register(char),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Instruction {
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
            letter().map(Operand::Register),
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
        sep_by(optional(instruction), spaces()),
        |f: Vec<Option<Instruction>>| f.into_iter().flatten().collect(),
    )
}

pub fn parse_instructions(input: &str) -> anyhow::Result<Vec<Instruction>> {
    let (result, _) = make_parser()
        .easy_parse(position::Stream::new(input))
        .map_err(|e| AocError::ParseError(format!("{}", e)))?;
    Ok(result)
}

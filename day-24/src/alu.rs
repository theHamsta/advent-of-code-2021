use crate::{
    instruction::{Instruction, Operand},
    op::Op,
    AocError,
};
use maplit::hashmap;
use std::{collections::HashMap, rc::Rc};

#[derive(Default, Debug)]
pub struct Alu {
    registers: HashMap<char, i64>,
}

impl Alu {
    pub fn run(&mut self, instructions: &[Instruction], input: &[i64]) -> anyhow::Result<()> {
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

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    fn get_value(&mut self, o: Operand) -> i64 {
        match o {
            Operand::Register(reg) => *self.registers.entry(reg).or_default(),
            Operand::Literal(value) => value,
        }
    }

    pub fn symbolic_execution(
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

    /// Get a reference to the alu's registers.
    pub fn registers(&self) -> &HashMap<char, i64> {
        &self.registers
    }

    pub fn register_mut(&mut self, name: char) -> &mut i64 {
        self.registers.entry(name).or_default()
    }

    pub fn register(&self, name: char) -> i64 {
        self.registers[&name]
    }
}

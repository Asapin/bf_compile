use crate::memory::Memory;
use std::{
    io::{Error, ErrorKind},
    str::Chars,
};
use text_io::read;

pub enum Instruction {
    NextCell,
    PrevCell,
    IncVal,
    DecVal,
    PrintVal,
    EnterVal,
    Loop { instructions: Vec<Instruction> },
}

pub struct VM {
    instructions: Vec<Instruction>,
    memory: Memory,
}

impl VM {
    pub fn run(input: &str) -> Result<(), Error> {
        let mut iter = input.chars();

        let instructions = VM::parse(&mut iter, false)?;
        let mut vm = VM {
            instructions,
            memory: Memory::new(),
        };

        vm.execute()
    }

    fn parse(iter: &mut Chars, inside_loop: bool) -> Result<Vec<Instruction>, Error> {
        let mut result = Vec::with_capacity(50);

        while let Some(c) = iter.next() {
            match c {
                '>' => result.push(Instruction::NextCell),
                '<' => result.push(Instruction::PrevCell),
                '+' => result.push(Instruction::IncVal),
                '-' => result.push(Instruction::DecVal),
                '.' => result.push(Instruction::PrintVal),
                ',' => result.push(Instruction::EnterVal),
                '[' => {
                    let inner_loop = VM::parse(iter, true)?;
                    result.push(Instruction::Loop {
                        instructions: inner_loop,
                    });
                }
                ']' => {
                    if inside_loop {
                        return Ok(result);
                    } else {
                        return Err(Error::new(ErrorKind::InvalidInput, "Unmatched ']'"));
                    }
                }
                _ => {
                    // skip
                }
            }
        }

        if inside_loop {
            Err(Error::new(ErrorKind::InvalidInput, "Unmatched '['"))
        } else {
            Ok(result)
        }
    }

    fn execute(&mut self) -> Result<(), Error> {
        VM::run_instructions(&self.instructions, &mut self.memory)
    }

    fn run_instructions(instructions: &[Instruction], memory: &mut Memory) -> Result<(), Error> {
        for instruction in instructions.iter() {
            match instruction {
                Instruction::NextCell => memory.next_cell()?,
                Instruction::PrevCell => memory.prev_cell()?,
                Instruction::IncVal => memory.inc_value()?,
                Instruction::DecVal => memory.dec_value()?,
                Instruction::PrintVal => {
                    let value = memory.read_value()? as char;
                    print!("{}", value);
                }
                Instruction::EnterVal => {
                    let value = read!();
                    memory.write_value(value)?;
                }
                Instruction::Loop { instructions } => {
                    let start_flag = memory.read_value()?;
                    if start_flag != 0 {
                        VM::run_loop(instructions, memory)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn run_loop(instructions: &[Instruction], memory: &mut Memory) -> Result<(), Error> {
        VM::run_instructions(instructions, memory)?;

        let end_flag = memory.read_value()?;
        if end_flag != 0 {
            VM::run_loop(instructions, memory)?;
        }

        Ok(())
    }
}

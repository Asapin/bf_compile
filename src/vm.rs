use crate::memory::Memory;
use std::io::Error;
use text_io::read;

pub enum Instruction {
    NextCell,
    PrevCell,
    IncVal,
    DecVal,
    PrintVal,
    ReadVal,
    Loop { instructions: Vec<Instruction> },
}

pub struct Machine {
    instructions: Vec<Instruction>,
    memory: Memory,
}

impl Machine {
    pub fn new(instructions: Vec<Instruction>) -> Machine {
        Machine {
            instructions,
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        Machine::run_instructions(&self.instructions, &mut self.memory)
    }

    fn run_instructions(instructions: &[Instruction], memory: &mut Memory) -> Result<(), Error> {
        for instruction in instructions.iter() {
            match instruction {
                Instruction::NextCell => memory.next_cell()?,
                Instruction::PrevCell => memory.prev_cell()?,
                Instruction::IncVal => memory.inc_value()?,
                Instruction::DecVal => memory.dec_value()?,
                Instruction::PrintVal => {
                    let value = memory.read_value()?;
                    print!("{}", value);
                }
                Instruction::ReadVal => {
                    let value = read!();
                    memory.write_value(value)?;
                }
                Instruction::Loop { instructions } => {
                    let start_flag = memory.read_value()?;
                    if start_flag != 0 {
                        Machine::run_loop(instructions, memory)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn run_loop(instructions: &[Instruction], memory: &mut Memory) -> Result<(), Error> {
        Machine::run_instructions(instructions, memory)?;

        let end_flag = memory.read_value()?;
        if end_flag != 0 {
            Machine::run_loop(instructions, memory)?;
        }

        Ok(())
    }
}

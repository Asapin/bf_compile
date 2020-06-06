use crate::{
    code_gen::CodeGen, enums::Command, lexer::Lexer, memory::Memory, optimizer::Optimizer,
};
use std::io::Error;
use std::io::Write;
use text_io::read;

pub struct VM {
    commands: Vec<Command>,
    memory: Memory,
}

impl VM {
    pub fn run(input: &str) -> Result<(), Error> {
        let ir = Lexer::tokenize(input)?;
        let commands = CodeGen::translate(&ir)?;
        let optimized_commands = Optimizer::optimize_ir(&commands);

        let mut vm = VM {
            commands: optimized_commands,
            memory: Memory::new(),
        };

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        vm.execute(&mut stdout)?;

        Ok(())
    }

    fn execute(&mut self, stdout: &mut std::io::StdoutLock) -> Result<(), Error> {
        VM::execute_commands(&self.commands, &mut self.memory, stdout)
    }

    fn execute_commands(
        commands: &[Command],
        memory: &mut Memory,
        stdout: &mut std::io::StdoutLock,
    ) -> Result<(), Error> {
        for command in commands.iter() {
            match command {
                Command::IncMemPointerByN { n } => memory.inc_mem_pointer_by(*n)?,
                Command::DecMemPointerByN { n } => memory.dec_mem_pointer_by(*n)?,
                Command::IncValByN { n } => memory.inc_value_by(*n)?,
                Command::DecValByN { n } => memory.dec_value_by(*n)?,
                Command::PrintVal => {
                    let value = memory.read_value()? as char;
                    write!(stdout, "{}", value)?;
                }
                Command::EnterVal => {
                    let value = read!();
                    memory.write_value(value)?;
                }
                Command::Loop {
                    commands: loop_commands,
                } => {
                    let start_flag = memory.read_value()?;
                    if start_flag != 0 {
                        VM::run_loop(loop_commands, memory, stdout)?;
                    }
                }
                Command::SetZero => memory.write_value(0)?,
                Command::FirstZeroByStep { step } => {
                    memory.inc_mem_pointer_until_zero_value(*step)?
                }
                Command::FirstZeroByStepReverse { step } => {
                    memory.dec_mem_pointer_until_zero_value(*step)?
                }
            }
        }

        Ok(())
    }

    fn run_loop(
        commands: &[Command],
        memory: &mut Memory,
        stdout: &mut std::io::StdoutLock,
    ) -> Result<(), Error> {
        VM::execute_commands(commands, memory, stdout)?;

        let end_flag = memory.read_value()?;
        if end_flag != 0 {
            VM::run_loop(commands, memory, stdout)?;
        }

        Ok(())
    }
}

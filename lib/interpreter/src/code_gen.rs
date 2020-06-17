use crate::enums::{Command, IrCommand};
use std::io::{Error, ErrorKind};

pub struct CodeGen {}

impl CodeGen {
    pub fn translate(ir: &[IrCommand]) -> Result<Vec<Command>, Error> {
        let mut seq_length = 0;
        let mut prev_token: Option<&IrCommand> = Option::None;
        let mut result = Vec::with_capacity(ir.len());

        for current_token in ir {
            match current_token {
                IrCommand::PrintVal => {
                    CodeGen::finish_sequence(&mut prev_token, seq_length, &mut result)?;
                    result.push(Command::PrintVal);

                    prev_token = Option::None;
                    seq_length = 0;
                }
                IrCommand::Loop { commands } => {
                    CodeGen::finish_sequence(&mut prev_token, seq_length, &mut result)?;
                    result.push(Command::Loop {
                        commands: CodeGen::translate(commands)?,
                    });

                    prev_token = Option::None;
                    seq_length = 0;
                }
                IrCommand::NextCell
                | IrCommand::PrevCell
                | IrCommand::IncVal
                | IrCommand::DecVal => {
                    if let Some(token) = prev_token {
                        if token == current_token {
                            seq_length += 1;
                        } else {
                            CodeGen::finish_sequence(&mut prev_token, seq_length, &mut result)?;

                            prev_token = Some(current_token);
                            seq_length = 1;
                        }
                    } else {
                        prev_token = Some(current_token);
                        seq_length = 1;
                    }
                }
            }
        }

        CodeGen::finish_sequence(&mut prev_token, seq_length, &mut result)?;

        Ok(result)
    }

    fn finish_sequence(
        prev_token: &mut Option<&IrCommand>,
        seq_length: usize,
        result: &mut Vec<Command>,
    ) -> Result<(), Error> {
        if let Some(token) = prev_token {
            match token {
                IrCommand::NextCell => result.push(Command::IncMemPointerByN { n: seq_length }),
                IrCommand::PrevCell => result.push(Command::DecMemPointerByN { n: seq_length }),
                IrCommand::IncVal => result.push(Command::IncValByN {
                    n: seq_length as u8,
                }),
                IrCommand::DecVal => result.push(Command::DecValByN {
                    n: seq_length as u8,
                }),
                _ => {
                    return Err(Error::new(ErrorKind::InvalidInput, "Should never happen"));
                }
            }
        }

        Ok(())
    }
}

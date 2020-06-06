use crate::enums::IrCommand;
use std::{
    io::{Error, ErrorKind},
    str::Chars,
};

pub struct Lexer {}

impl Lexer {
    pub fn tokenize(input: &str) -> Result<Vec<IrCommand>, Error> {
        let mut iter = input.chars();
        Lexer::do_tokenization(&mut iter, false)
    }

    fn do_tokenization(iter: &mut Chars, inside_loop: bool) -> Result<Vec<IrCommand>, Error> {
        let mut result = Vec::with_capacity(50);

        while let Some(c) = iter.next() {
            match c {
                '>' => result.push(IrCommand::NextCell),
                '<' => result.push(IrCommand::PrevCell),
                '+' => result.push(IrCommand::IncVal),
                '-' => result.push(IrCommand::DecVal),
                '.' => result.push(IrCommand::PrintVal),
                ',' => result.push(IrCommand::EnterVal),
                '[' => {
                    let inner_loop = Lexer::do_tokenization(iter, true)?;
                    result.push(IrCommand::Loop {
                        commands: inner_loop,
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
}

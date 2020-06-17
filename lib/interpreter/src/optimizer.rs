use crate::enums::Command;

pub struct Optimizer {}

impl Optimizer {
    pub fn optimize_ir(ir: &[Command]) -> Vec<Command> {
        ir.iter()
            .map(|command| match command {
                Command::Loop { commands } => {
                    if commands.len() == 1 {
                        let c: &Command = commands.get(0).unwrap();
                        match c {
                            Command::IncMemPointerByN { n } => {
                                Command::FirstZeroByStep { step: *n }
                            }
                            Command::DecMemPointerByN { n } => {
                                Command::FirstZeroByStepReverse { step: *n }
                            }
                            Command::DecValByN { n: _ } => Command::SetZero,
                            Command::Loop { commands } => Command::Loop {
                                commands: Optimizer::optimize_ir(commands),
                            },
                            _ => command.clone(),
                        }
                    } else {
                        Command::Loop {
                            commands: Optimizer::optimize_ir(commands),
                        }
                    }
                }
                _ => command.clone(),
            })
            .collect()
    }
}

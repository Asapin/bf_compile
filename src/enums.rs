#[derive(PartialEq)]
pub enum IrCommand {
    NextCell,
    PrevCell,
    IncVal,
    DecVal,
    PrintVal,
    EnterVal,
    Loop { commands: Vec<IrCommand> },
}

#[derive(PartialEq, Debug, Clone)]
pub enum Command {
    IncMemPointerByN { n: usize },
    DecMemPointerByN { n: usize },
    IncValByN { n: u8 },
    DecValByN { n: u8 },
    PrintVal,
    EnterVal,
    Loop { commands: Vec<Command> },
    SetZero,
    FirstZeroByStep { step: usize },
    FirstZeroByStepReverse { step: usize },
}

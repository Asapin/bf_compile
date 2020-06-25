#[derive(PartialEq)]
pub enum IrCommand {
    NextCell,
    PrevCell,
    IncVal,
    DecVal,
    PrintVal,
    Loop { commands: Vec<IrCommand> },
}

#[derive(PartialEq, Clone)]
pub enum Command {
    IncMemPointerByN { n: usize },
    DecMemPointerByN { n: usize },
    IncValByN { n: u8 },
    DecValByN { n: u8 },
    PrintVal,
    Loop { commands: Vec<Command> },
    SetZero,
    FirstZeroByStep { step: usize },
    FirstZeroByStepReverse { step: usize },
}

mod memory;
mod vm;

use vm::{Instruction, Machine};

fn main() {
    let instructions = vec![
        Instruction::ReadVal,
        Instruction::NextCell,
        Instruction::ReadVal,
        Instruction::PrevCell,
        Instruction::Loop {
            instructions: vec![
                Instruction::DecVal,
                Instruction::NextCell,
                Instruction::IncVal,
                Instruction::PrevCell,
            ],
        },
        Instruction::NextCell,
        Instruction::PrintVal,
    ];

    let mut machine = Machine::new(instructions);
    machine.run().unwrap();
}

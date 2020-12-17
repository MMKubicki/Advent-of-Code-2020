mod instruction;

pub use instruction::{parse_instruction_list, Instruction, Mask, Write};

use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Memory {
    mask: Mask,
    mem: HashMap<usize, usize>,
}

pub fn apply_list(
    memory: &mut Memory,
    apply_func: impl Fn(&mut Memory, &Instruction),
    list: &[Instruction],
) {
    for instruction in list {
        apply_func(memory, instruction)
    }
}

impl Memory {
    pub fn apply_v1(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask { mask } => self.mask = *mask,
            Instruction::Write { write } => {
                self.mem.insert(write.to, self.mask.apply_v1(write.value));
            }
        }
    }

    pub fn apply_v2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask { mask } => self.mask = *mask,
            Instruction::Write { write } => {
                for new_to in self.mask.apply_v2(write.to) {
                    self.mem.insert(new_to, write.value);
                }
            }
        }
    }

    pub fn sum(&self) -> usize {
        self.mem.values().sum()
    }
}

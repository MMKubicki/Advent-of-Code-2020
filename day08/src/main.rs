mod boot_code;

use std::borrow::Cow;
use std::fs;

use boot_code::{parse_instruction_list, Instruction, Machine, Sign, TermReason};

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();
    let content = fs::read_to_string(options.input)?;

    let instructions = parse_instruction_list(&content)?;

    // part 1
    let mut machine_1 = Machine::default();
    let part1_acc = match machine_1.run_till_term(&instructions) {
        TermReason::Loop(acc) => acc,
        _ => unreachable!("according to task"),
    };
    println!("Acc before first loop: {}", part1_acc);

    // part 2
    let fixed_instructions = fix_looped_instructions_bruteforce(&instructions);
    let mut machine_2 = Machine::default();
    let part2_acc = match machine_2.run_till_term(&fixed_instructions) {
        TermReason::End(acc) => acc,
        _ => unreachable!("according to task"),
    };
    println!("Acc at end with fixed instructions: {}", part2_acc);

    Ok(())
}

fn get_cow_instructions(instructions: &[Instruction]) -> Vec<Cow<Instruction>> {
    instructions.iter().map(|v| Cow::Borrowed(v)).collect()
}

// just brute force the solution

fn fix_looped_instructions_bruteforce(instructions: &[Instruction]) -> Vec<Cow<Instruction>> {
    let mut mod_instruct = get_cow_instructions(instructions);

    // Get the index from all nop and jump instructions
    // change them to the other while keeping the values and try if it runs till end
    // if it runs the change is a solution
    let possible_positions = instructions
        .iter()
        .enumerate()
        .filter_map(|(idx, ins)| match ins {
            Instruction::Nop(_, _) => try_change(&mod_instruct, idx),
            Instruction::Jmp(_, _) => try_change(&mod_instruct, idx),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(possible_positions.len(), 1);

    // apply change
    let pos = *possible_positions.first().unwrap();
    mod_instruct[pos] = Cow::Owned(match instructions[pos] {
        Instruction::Jmp(sign, value) => Instruction::Nop(sign, value),
        Instruction::Nop(sign, value) => Instruction::Jmp(sign, value),
        _ => unreachable!(),
    });

    mod_instruct
}

fn try_change(instructions: &[Cow<Instruction>], idx: usize) -> Option<usize> {
    let mut instructions = Cow::from(instructions);

    // apply change
    instructions.to_mut()[idx] = Cow::Owned(match &instructions[idx].as_ref() {
        Instruction::Jmp(sign, value) => Instruction::Nop(*sign, *value),
        Instruction::Nop(sign, value) => Instruction::Jmp(*sign, *value),
        _ => return None,
    });

    // try
    match Machine::default().run_till_term(&instructions) {
        TermReason::Loop(_) => None,
        TermReason::End(_) => Some(idx),
    }
}

// some parts from trying to solve part 2 with analysing the Instructions that lead up to a back jump
// ##################

fn fix_looped_instructions(instructions: &[Instruction]) -> Vec<Cow<Instruction>> {
    let mut result = get_cow_instructions(instructions);

    let mut _jump_back_positions = instructions
        .iter()
        .enumerate()
        .filter(|(_, i)| matches!(i, Instruction::Jmp(Sign::Minus, _)))
        .map(|(c, _)| c)
        .rev()
        .filter_map(|pos| check_replace_jump_back(instructions, pos))
        .map(|val| println!("{}", val))
        .collect::<()>();

    //TODO: Modify result

    result
}

fn check_replace_jump_back(instructions: &[Instruction], jump_pos: usize) -> Option<usize> {
    let mut machine = Machine::new(jump_pos);

    machine.run_instruction(instructions[jump_pos]);

    let start_pos = machine.get_pc();

    if !matches!(machine.run_till_term(instructions), TermReason::Loop(_)) {
        return None;
    }

    for pc in start_pos..(jump_pos - 1) {
        match instructions[pc] {
            Instruction::Nop(Sign::Plus, value) => {
                if pc + value > jump_pos {
                    return Some(pc);
                }
            }
            Instruction::Jmp(Sign::Plus, value) => {
                if pc + value < jump_pos {
                    return Some(pc);
                }
            }
            // Ignore Nop/Jump with negative value for now
            _ => {}
        }
    }

    None
}

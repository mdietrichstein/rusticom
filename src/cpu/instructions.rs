use super::MOS6502Cpu;
use super::types::OpCode;
use super::types::NoArgumentInstructionOp;
use super::types::SingleArgumentInstructionOp;
use super::types::AddressingMode;
use super::types::InstructionMap;

#[derive(Clone, Copy)]
pub enum Instruction {
    Invalid,
    NoArgument { op_code: OpCode, mnemonic: &'static str, operation: &'static NoArgumentInstructionOp, cycles: u8 },
    SingleArgument { op_code: OpCode, mnemonic: &'static str, operation: &'static SingleArgumentInstructionOp, adressing_mode: &'static AddressingMode, cycles: u8 },
}

pub fn create_instruction_map() -> InstructionMap {
    // Define all instructions as invalid and the add the implemented ones, one-by-one
    
    // Addressing Modes
    let IMM = &MOS6502Cpu::addressing_mode_immediate;
    let ZP = &MOS6502Cpu::addressing_mode_zero_page;
    let ZPX = &MOS6502Cpu::addressing_mode_zero_page_x;
    let ZPY = &MOS6502Cpu::addressing_mode_zero_page_y;

    // Operations
    let LDA = &MOS6502Cpu::load_accumulator;

    // Instruction Set
    let mut I = [Instruction::Invalid; 256];

    I[0xA9] =  Instruction::SingleArgument {
        op_code: 0xA9,
        mnemonic: "LDA",
        operation: LDA,
        adressing_mode: IMM,
        cycles: 2,
    };
    
    I
}

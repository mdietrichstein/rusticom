use super::MOS6502Cpu;
use super::types::OpCode;
use super::types::InstructionOp;
use super::types::AdressingMode;
use super::types::InstructionMap;

#[derive(Clone, Copy)]
pub enum Instruction {
    Invalid,
    Valid { op_code: OpCode, mnemonic: &'static str, cycles: u8, operation: &'static InstructionOp, adressing_mode: &'static AdressingMode },
}

pub fn create_instruction_map() -> InstructionMap {
    // Define all instructions as invalid and the add the implemented ones, one-by-one
    let mut i = [Instruction::Invalid; 256];

    i[0xA9] =  Instruction::Valid {
        op_code: 0xA9,
        mnemonic: "LDA",
        cycles: 2,
        adressing_mode: &MOS6502Cpu::adressing_mode_immediate,
        operation: &MOS6502Cpu::load_accumulator,
    };
    
    i
}
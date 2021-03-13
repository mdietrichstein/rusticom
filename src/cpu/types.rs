use super::MOS6502Cpu;
use super::instructions::Instruction;

pub type InstructionMap = [Instruction; 256];
pub type OpCode = u8;
pub type Status = u8;
pub type InstructionOp = dyn Fn(&MOS6502Cpu);
pub type AdressingMode = dyn Fn(&MOS6502Cpu) -> u8;
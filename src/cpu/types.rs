use super::MOS6502Cpu;
use super::instructions::Instruction;

pub type InstructionMap = [Instruction; 256];
pub type OpCode = u8;
pub type Status = u8;
pub type NoArgumentInstructionOp = Fn(&mut MOS6502Cpu);
pub type SingleArgumentInstructionOp = Fn(&mut MOS6502Cpu, u8);
pub type AdressingMode = Fn(&mut MOS6502Cpu) -> u8;
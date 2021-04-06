use super::MOS6502Cpu;
use super::instructions::Instruction;

pub type Address = u16;

pub type InstructionMap = [Instruction; 256];
pub type OpCode = u8;
pub type Status = u8;
pub type AddressLookupResult = (Address, bool);
pub type AddressingMode = dyn Fn(&mut MOS6502Cpu) -> AddressLookupResult;
pub type SingleArgumentInstructionOp = dyn Fn(&mut MOS6502Cpu, &'static AddressingMode);
pub type NoArgumentInstructionOp = dyn Fn(&mut MOS6502Cpu);
mod types;
mod memory;
mod bus;
mod instructions;

use self::types::OpCode;
use self::types::Status;
use self::types::NoArgumentInstructionOp;
use self::types::SingleArgumentInstructionOp;
use self::types::AddressingMode;
use self::types::InstructionMap;
use self::types::AddressLookupResult;
use self::instructions::Instruction;
use self::instructions::create_instruction_map;
use self::memory::Memory;
use self::memory::IntoAddress;
use self::bus::Bus;
    
pub struct MOS6502Cpu {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    status: Status,
    instructions: InstructionMap,
    pub bus: Bus,
}

impl Memory for MOS6502Cpu {
    fn write<A: IntoAddress>(&mut self, address: A, data: u8) {
        self.bus.write(address, data);
    }

    fn read<A: IntoAddress>(&self, address: A) -> u8 {
        self.bus.read(address)
    }
}

impl MOS6502Cpu {
    pub fn new() -> Self {
        MOS6502Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            status: 0,
            bus: Bus::new(),
            instructions: create_instruction_map()
        }
    }

    pub fn run(&mut self, program: Vec<u8>) {
        self.pc = 0;

        loop {
            let op = program[self.pc as usize];
            self.pc += 1;
    
            let instruction = self.instructions[op as usize];

            match instruction {
                Instruction::Invalid => panic!("Invalid instruction: 0x{:02X}", op),
                Instruction::NoArgument { op_code, mnemonic, cycles, operation } =>  {
                    (operation)(self)
                },
                Instruction::SingleArgument { op_code, mnemonic, cycles, operation, addressing_mode } =>  {
                    let (argument, pages_crossed) = addressing_mode(self);
                    (operation)(self, argument)
                }
            }
        }
    }
        
    //#region Instructions

    fn load_accumulator(&mut self, argument: u8) {
        self.a = argument;
        println!("LDA")
    }


    fn and(&mut self, argument: u8) {
        self.a &= argument
    }

    //#endregion

    //#region Addressing Modes

    fn addressing_mode_immediate(&mut self) -> AddressLookupResult {
        let argument = self.read(self.pc);
        self.pc += 1;

        (argument, false)
    }

    fn addressing_mode_zero_page(&mut self) -> AddressLookupResult {
        let address = self.read(self.pc);
        let argument = self.read(address);
        self.pc += 1;

        (argument, false)
    }

    fn addressing_mode_zero_page_x(&mut self) -> AddressLookupResult {
        let address = self.read(self.pc);
        let argument = self.read(address);

        self.pc += 1;

        (argument, false)
    }

    fn addressing_mode_zero_page_y(&mut self) -> AddressLookupResult {
        let address = self.read(self.pc);
        let argument = self.read(address.wrapping_add(self.y));

        self.pc += 1;

        (argument, false)
    }

    fn addressing_mode_absolute(&mut self) -> AddressLookupResult {
        let low_byte = self.read(self.pc);
        let high_byte = self.read(self.pc + 1);

        self.pc += 2;

        let address = ((high_byte as u16) << 8) | low_byte as u16;
        let argument = self.read(address);

        (argument, false)
    }

    fn addressing_mode_absolute_x(&mut self) -> AddressLookupResult {
        let low_byte = self.read(self.pc);
        let high_byte = self.read(self.pc + 1);

        self.pc += 2;

        let address = (((high_byte as u16) << 8) | low_byte as u16) + self.x as u16;
        let argument = self.read(address);
        let pages_crossed = high_byte != (address >> 8) as u8;

        (argument, pages_crossed)
    }

    fn addressing_mode_absolute_y(&mut self) -> AddressLookupResult {
        let low_byte = self.read(self.pc);
        let high_byte = self.read(self.pc + 1);

        self.pc += 2;

        let address = (((high_byte as u16) << 8) | low_byte as u16) + self.y as u16;
        let argument = self.read(address);
        let pages_crossed = high_byte != (address >> 8) as u8;

        (argument, pages_crossed)
    }

    fn addressing_mode_indirect(&mut self) -> AddressLookupResult {
        let pointer_low_byte = self.read(self.pc);
        let pointer_high_byte = self.read(self.pc + 1);

        self.pc += 2;

        let pointer = ((pointer_high_byte as u16) << 8) | pointer_low_byte as u16;

        let low_byte = self.read(pointer);

        let high_byte =  if pointer_low_byte == 0xFF {
            // Simulate CPU Bug: if the low byte is 0xFF, the carry bit is not propagated into the high byte when adding +1 to the pointer
            self.read(pointer & 0xFF00)
        } else {            
            self.read(pointer + 1)        
        };

        let address = ((high_byte as u16) << 8) | low_byte as u16;

        let argument = self.read(address);

        (argument, false)
    }

    fn addressing_mode_indirect_x(&mut self) -> AddressLookupResult {
        let zero_page_pointer = self.read(self.pc).wrapping_add(self.x);

        // Both bytes must be located in page zero
        let low_byte = self.read(zero_page_pointer);
        let high_byte = self.read(zero_page_pointer.wrapping_add(1));

        self.pc += 1;

        let address = ((high_byte as u16) << 8)  | low_byte as u16;

        let argument = self.read(address);
        (argument, false)
    }

    fn addressing_mode_indirect_y(&mut self) -> AddressLookupResult {
        let zero_page_pointer = self.read(self.pc);
        let low_byte = self.read(zero_page_pointer);
        let high_byte = self.read(zero_page_pointer.wrapping_add(1));

        self.pc += 1;

        let address = (((high_byte as u16) << 8) | low_byte as u16) + self.y as u16;
        let argument = self.read(address);
        let pages_crossed = high_byte != (address >> 8) as u8;

        (argument, pages_crossed)
    }

    // fn addressing_mode_relative(&mut self) -> AddressLookupResult {
    //     // cast to i8 since offset can be negative
    //     let relative_pc_offset = self.memory[self.pc as usize] as i8;
        
    //     // increment program counter before adding the offset
    //     self.pc += 1;

    //     let address = ((self.pc as i32) + (relative_pc_offset as i32)) as usize;
        
    //     (address, false)
    // }

    //#endregion
}
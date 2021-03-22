use super::memory::Memory;
use super::memory::IntoAddress;
use std::ops::RangeInclusive;

const MEMORY_RANGE_RAM: RangeInclusive<u16> = 0 ..= 0x1FFF;

pub struct Bus {
    // RAM device
    memory: [u8; 0xFFFF]
    
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            memory: [0; 0xFFFF]
        }
    }
}

impl Memory for Bus {
    fn write<A: IntoAddress>(&mut self, address: A, data: u8) {
        self.memory[address.into_address() as usize] = data;
    }

    fn read<A: IntoAddress>(&self, address: A) -> u8 {
        self.memory[address.into_address() as usize]
    }
}
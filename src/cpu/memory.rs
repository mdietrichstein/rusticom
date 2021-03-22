use super::types::Address;

pub trait IntoAddress {
    fn into_address(self) -> Address;
}

impl IntoAddress for u8 {
    fn into_address(self) -> Address {
        self as Address
    }
}

impl IntoAddress for u16 {
    fn into_address(self) -> Address {
        self as Address
    }
}

pub trait Memory {
    fn write<A: IntoAddress>(&mut self, address: A, data: u8);
    fn read<A: IntoAddress>(&self, address: A) -> u8;
}
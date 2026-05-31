use crate::status::Status;

mod addressing;
mod instruction;
mod memory;
mod status;

#[derive()]
pub struct MOS6502 {
    a: u8,
    p: Status,
    pc: u16,
    s: u8,
    x: u8,
    y: u8,
}

impl Default for MOS6502 {
    fn default() -> Self {
        Self {
            a: 0x00,
            p: Status::CLR,
            pc: 0x00,
            s: 0x00,
            x: 0x00,
            y: 0x00,
        }
    }
}

impl MOS6502 {
    pub fn new(a: u8, p: Status, pc: u16, s: u8, x: u8, y: u8) -> Self {
        Self { a, p, pc, s, x, y }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let machine = MOS6502::default();

        assert_eq!(machine.a, 0)
    }

    #[test]
    fn reset() {
        let mut machine = MOS6502::default();
        machine.reset();
        assert_eq!(machine.pc, 0)
    }
}

use crate::memory::Memory;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressMode {
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Relative,
    Indirect,
    IndirectX,
    IndirectY,
}

pub fn resolve(mode: &AddressMode, memory: &Memory, pc: u16, a: u8, x: u8, y: u8) -> u16 {
    match mode {
        AddressMode::Accumulator => a as u16,
        AddressMode::Immediate => pc + 1,
        AddressMode::ZeroPage => memory.read(pc + 1) as u16,
        AddressMode::ZeroPageX => (memory.read(pc + 1).wrapping_add(x)) as u16,
        AddressMode::ZeroPageY => (memory.read(pc + 1).wrapping_add(y)) as u16,
        AddressMode::Absolute => memory.read_word(pc + 1),
        AddressMode::AbsoluteX => memory.read_word(pc + 1) + (x as u16),
        AddressMode::AbsoluteY => memory.read_word(pc + 1) + (y as u16),
        AddressMode::Relative => {
            let addr = memory.read(pc + 1) as i8;
            let addr = pc.wrapping_add(addr as u16);
            addr
        }
        AddressMode::Indirect => {
            let addr = memory.read_word(pc + 1);
            let addr = memory.read_word(addr);
            addr
        }
        AddressMode::IndirectX => {
            let addr = memory.read(pc + 1).wrapping_add(x) as u16;
            let addr = memory.read_word(addr);
            addr
        }
        AddressMode::IndirectY => {
            let addr = memory.read(pc + 1) as u16; // single byte -> zero page address
            let addr = memory.read_word(addr) + y as u16;
            addr
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::addressing::resolve;

    struct TestState {
        memory: Memory,
        pc: u16,
        a: u8,
        x: u8,
        y: u8,
    }

    fn setup() -> TestState {
        let mut memory = Memory::default();

        memory.write(0x0401, 0x21);
        memory.write(0x0402, 0xFF);
        memory.write(0x0021, 0x41);
        memory.write(0x00FF, 0x42);
        //add bytes later for memory addressing
        TestState {
            memory: memory,
            pc: 0x0401,
            a: 0x47,
            x: 0x51,
            y: 0x77,
        }
    }

    /**
     * Accumulator:
     * Just the accumulator value itself
     */
    #[test]
    fn resolve_accumulator() {
        let TestState { memory, a, .. } = setup();

        let addr = resolve(&AddressMode::Accumulator, &memory, 0, a, 0, 0);

        assert_eq!(addr as usize, a as usize)
    }

    /**
     * Immediate
     * The byte immediately after the opcode, so pc + 1
     */
    #[test]
    fn resolve_immediate() {
        let TestState { memory, pc, .. } = setup();

        let addr = resolve(&AddressMode::Immediate, &memory, pc, 0, 0, 0);

        assert_eq!(addr as usize, (pc + 1) as usize)
    }

    /**
     * ZeroPage
     * read the byte at pc + 1: that byte is the address.
     * So if pc + 1 contains 0x42, the address is 0x0042.
     */
    #[test]
    fn resolve_zero_page() {
        let TestState { memory, pc, .. } = setup();

        let addr = resolve(&AddressMode::ZeroPage, &memory, pc, 0, 0, 0);

        let val = memory.read(pc + 1);

        assert_eq!(addr as usize, val as usize)
    }

    /**
     * ZeroPageX - same as ZeroPage but add X to the result. Wraps at 255 so it stays in the zero page.
     */
    #[test]
    fn resolve_zero_page_x() {
        let TestState { memory, pc, x, .. } = setup();

        let addr = resolve(&AddressMode::ZeroPageX, &memory, pc, 0, x, 0);

        //this needs to add X with wrap around.
        let val = memory.read(pc + 1).wrapping_add(x);

        assert_eq!(addr as usize, val as usize)
    }

    /**
     * ZeroPageY - same as ZeroPage but add Y to the result. Wraps at 255 so it stays in the zero page.
     */
    #[test]
    fn resolve_zero_page_y() {
        let TestState { memory, pc, y, .. } = setup();

        let addr = resolve(&AddressMode::ZeroPageY, &memory, pc, 0, 0, y);

        //this needs to add X with wrap around.
        let val = memory.read(pc + 1).wrapping_add(y);

        assert_eq!(addr as usize, val as usize)
    }

    /**
     * Absolute - read two bytes starting at pc + 1, combine them little-endian into a 16-bit address.
     */
    #[test]
    fn resolve_absolute() {
        let TestState { memory, pc, .. } = setup();

        let addr = resolve(&AddressMode::Absolute, &memory, pc, 0, 0, 0);

        //this needs to add X with wrap around.
        let val = memory.read_word(pc + 1);

        assert_eq!(addr as usize, val as usize)
    }

    /**
     * AbsoluteX - same as Absolute but add X to the result.
     */
    #[test]
    fn resolve_absolute_x() {
        let TestState { memory, pc, x, .. } = setup();

        let addr = resolve(&AddressMode::AbsoluteX, &memory, pc, 0, x, 0);

        let val = memory.read_word(pc + 1) + (x as u16);

        assert_eq!(addr as usize, val as usize)
    }

    /**
     * AbsoluteY - same as Absolute but add Y to the result.
     */
    #[test]
    fn resolve_absolute_y() {
        let TestState { memory, pc, y, .. } = setup();

        let addr = resolve(&AddressMode::AbsoluteY, &memory, pc, 0, 0, y);

        let val = memory.read_word(pc + 1) + (y as u16);

        assert_eq!(addr as usize, val as usize)
    }

    /**
     * Relative - read the byte at pc + 1 as a signed offset. Add it to PC to get the target. Used only by branch instructions.
     */
    #[test]
    fn resolve_relative() {
        let TestState { memory, pc, .. } = setup();

        let addr = resolve(&AddressMode::Relative, &memory, pc, 0, 0, 0);

        let val = memory.read(pc + 1) as i8;
        let val = pc.wrapping_add(val as u16);

        assert_eq!(addr as usize, val as usize)
    }

    /**
     * Indirect - read a 16-bit address from pc + 1, then read the actual target address from that location. A pointer to a pointer essentially.
     */
    #[test]
    fn resolve_indirect() {
        let TestState { memory, pc, .. } = setup();

        let addr = resolve(&AddressMode::Indirect, &memory, pc, 0, 0, 0);

        let val = memory.read_word(pc + 1);
        let val = memory.read_word(val);

        assert_eq!(addr as usize, val as usize)
    }

    /**
     * IndirectX - read the byte at pc + 1, add X to it (wrapping, zero page), then read the 16-bit address stored at that zero page location.
     */
    #[test]
    fn resolve_indirect_x() {
        let TestState { memory, pc, x, .. } = setup();

        let addr = resolve(&AddressMode::IndirectX, &memory, pc, 0, x, 0);

        let val = memory.read(pc + 1).wrapping_add(x) as u16;
        let val = memory.read_word(val);

        assert_eq!(addr as usize, val as usize)
    }

    /**
     * IndirectY - read the byte at pc + 1, add Y to it (wrapping, zero page), then read the 16-bit address stored at that zero page location.
     */
    #[test]
    fn resolve_indirect_y() {
        let TestState { memory, pc, y, .. } = setup();

        let addr = resolve(&AddressMode::IndirectY, &memory, pc, 0, 0, y);

        let val = memory.read(pc + 1) as u16; // single byte -> zero page address
        let val = memory.read_word(val) + y as u16;

        assert_eq!(addr as usize, val as usize)
    }

    // TODO: page boundary bug
}

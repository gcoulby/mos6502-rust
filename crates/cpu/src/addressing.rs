pub enum AddressMode {
    Implied,
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

// pub fn resolveAddress(&self, cpu: &MOS6502, memory: &Memory) -> u16 {
//     match self {
//         AddressMode::Immediate => cpu.pc + 1,
//         AddressMode::ZeroPage => memory.read(cpu.pc + 1) as u16,
//         AddressMode::ZeroPageX => memory.read(cpu.pc + 1).wrapping_add(cpu.x) as u16,
//         // etc
//     }
// }

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Status: u8 {
        const NEGATIVE  = 1 << 7; // 1000 0000 | 0x80 | 128
        const OVERFLOW  = 1 << 6; // 0100 0000 | 0x40 | 64
        const BREAK     = 1 << 4; // 0001 0000 | 0x10 | 16
        const DECIMAL   = 1 << 3; // 0000 1000 | 0x08 | 8
        const INTERRUPT = 1 << 2; // 0000 0100 | 0x04 | 4
        const ZERO      = 1 << 1; // 0000 0010 | 0x02 | 2
        const CARRY     = 1 << 0; // 0000 0001 | 0x01 | 1
        const CLR       = 1 << 5; // 0010 0000 | 0x20 | 32
    }
}

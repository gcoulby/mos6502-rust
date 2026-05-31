pub struct Memory {
    data: [u8; 65536],
}

impl Default for Memory {
    fn default() -> Self {
        Self { data: [0; 65536] }
    }
}

impl Memory {
    pub fn new(data: [u8; 65536]) -> Self {
        Self { data }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    /**
     * Example: ADDR    = 0x00F0
     *          0x00F0  = 0x42
     *          0x00F1  = 0x44
     *
     * |           | base 16   | base 2                | base 10 |
     * |-----------|-----------|-----------------------|---------|
     * |0x00F0     | 42        | 0100 0010             | 62      |
     * |0x00F1     | 44        | 0100 0100             | 68      |
     * |44 << 8    | 4400      | 0100 0100 0000 0000   | 17408   |
     * |4400 + 42  | 4442      | 0100 0100 0100 0010   | 17474   |
     */
    pub fn read_word(&self, addr: u16) -> u16 {
        let lo = self.data[addr as usize] as u16;
        let hi = self.data[(addr + 1) as usize] as u16;
        lo | (hi << 8)
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.data[addr as usize] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let memory = Memory::default();

        assert_eq!(memory.read(65535), 0x00)
    }

    #[test]
    fn write() {
        let mut memory = Memory::default();
        memory.write(3000, 0xFA);

        assert_eq!(memory.data[3000], 0xFA);
    }

    #[test]
    fn write_then_read() {
        let mut memory = Memory::default();
        memory.write(3000, 0xFA);

        assert_eq!(memory.data[3000], 0xFA);

        let val = memory.read(3000);

        assert_eq!(val, 0xFA);
    }

    #[test]
    fn read_word() {
        let mut memory = Memory::default();
        memory.write(0x00F0, 0x42);
        memory.write(0x00F1, 0x44);

        let val = memory.read_word(0x00F0);

        assert_eq!(val, 0x4442);
    }
}

use crate::{addressing::AddressMode, instruction};

pub struct DecodedInstruction {
    instruction: Instruction,
    address_mode: Option<AddressMode>,
    size: u8,
    controls_pc: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Instruction {
    // ADC - Add with Carry
    AdcIm = 0x69,
    AdcZp = 0x65,
    AdcZpx = 0x75,
    AdcAbs = 0x6D,
    AdcAbsx = 0x7D,
    AdcAbsy = 0x79,
    AdcIndx = 0x61,
    AdcIndy = 0x71,
    // AND - Logical AND
    AndIm = 0x29,
    AndZp = 0x25,
    AndZpx = 0x35,
    AndAbs = 0x2D,
    AndAbsx = 0x3D,
    AndAbsy = 0x39,
    AndIndx = 0x21,
    AndIndy = 0x31,
    // ASL - Arithmetic Shift Left
    AslA = 0x0A,
    AslZp = 0x06,
    AslZpx = 0x16,
    AslAbs = 0x0E,
    AslAbsx = 0x1E,
    // BCC - Branch if Carry Clear
    BccRel = 0x90,
    // BCS - Branch if Carry Set
    BcsRel = 0xB0,
    // BEQ - Branch if Equal
    BeqRel = 0xF0,
    // BIT
    BitZp = 0x24,
    BitAbs = 0x2C,
    // BMI - Branch if Negative
    BmiRel = 0x30,
    // BNE - Branch if Not Equal
    BneRel = 0xD0,
    // BPL - Branch if Positive
    BplRel = 0x10,
    // BVC - Branch if Overflow Clear
    BvcRel = 0x50,
    // BVS - Branch if Overflow Set
    BvsRel = 0x70,
    // BRK - Break
    Brk = 0x00,
    // CLC - Clear Carry Flag
    Clc = 0x18,
    // CLD - Clear Decimal Flag
    Cld = 0xD8,
    // CLI - Clear Interrupt Disable Flag
    Cli = 0x58,
    // CLV - Clear Overflow Flag
    Clv = 0xB8,
    // CMP - Compare
    CmpIm = 0xC9,
    CmpZp = 0xC5,
    CmpZpx = 0xD5,
    CmpAbs = 0xCD,
    CmpAbsx = 0xDD,
    CmpAbsy = 0xD9,
    CmpIndx = 0xC1,
    CmpIndy = 0xD1,
    // CPX - Compare X Register
    CpxIm = 0xE0,
    CpxZp = 0xE4,
    CpxAbs = 0xEC,
    // CPY - Compare Y Register
    CpyIm = 0xC0,
    CpyZp = 0xC4,
    CpyAbs = 0xCC,
    // DEC - Decrement Memory
    DecZp = 0xC6,
    DecZpx = 0xD6,
    DecAbs = 0xCE,
    DecAbsx = 0xDE,
    // DEX - Decrement X Register
    Dex = 0xCA,
    // DEY - Decrement Y Register
    Dey = 0x88,
    // EOR - Exclusive OR
    EorIm = 0x49,
    EorZp = 0x45,
    EorZpx = 0x55,
    EorAbs = 0x4D,
    EorAbsx = 0x5D,
    EorAbsy = 0x59,
    EorIndx = 0x41,
    EorIndy = 0x51,
    // INC - Increment Memory
    IncZp = 0xE6,
    IncZpx = 0xF6,
    IncAbs = 0xEE,
    IncAbsx = 0xFE,
    // INX - Increment X Register
    Inx = 0xE8,
    // INY - Increment Y Register
    Iny = 0xC8,
    // JMP - Jump
    JmpAbs = 0x4C,
    JmpInd = 0x6C,
    // JSR - Jump to Subroutine
    Jsr = 0x20,
    // LDA - Load Accumulator
    LdaIm = 0xA9,
    LdaZp = 0xA5,
    LdaZpx = 0xB5,
    LdaAbs = 0xAD,
    LdaAbsx = 0xBD,
    LdaAbsy = 0xB9,
    LdaIndx = 0xA1,
    LdaIndy = 0xB1,
    // LDX - Load X Register
    LdxIm = 0xA2,
    LdxZp = 0xA6,
    LdxZpy = 0xB6,
    LdxAbs = 0xAE,
    LdxAbsy = 0xBE,
    // LDY - Load Y Register
    LdyIm = 0xA0,
    LdyZp = 0xA4,
    LdyZpx = 0xB4,
    LdyAbs = 0xAC,
    LdyAbsx = 0xBC,
    // LSR - Logical Shift Right
    LsrA = 0x4A,
    LsrZp = 0x46,
    LsrZpx = 0x56,
    LsrAbs = 0x4E,
    LsrAbsx = 0x5E,
    // NOP - No Operation
    Nop = 0xEA,
    // ORA - Logical OR
    OraIm = 0x09,
    OraZp = 0x05,
    OraZpx = 0x15,
    OraAbs = 0x0D,
    OraAbsx = 0x1D,
    OraAbsy = 0x19,
    OraIndx = 0x01,
    OraIndy = 0x11,
    // PHA - Push Accumulator to Stack
    Pha = 0x48,
    // PHP - Push Processor Status to Stack
    Php = 0x08,
    // PLA - Pull Accumulator from Stack
    Pla = 0x68,
    // PLP - Pull Processor Status from Stack
    Plp = 0x28,
    // ROL - Rotate Left
    RolA = 0x2A,
    RolZp = 0x26,
    RolZpx = 0x36,
    RolAbs = 0x2E,
    RolAbsx = 0x3E,
    // ROR - Rotate Right
    RorA = 0x6A,
    RorZp = 0x66,
    RorZpx = 0x76,
    RorAbs = 0x6E,
    RorAbsx = 0x7E,
    // RTI - Return from Interrupt
    Rti = 0x40,
    // RTS - Return from Subroutine
    Rts = 0x60,
    // SBC - Subtract with Carry
    SbcIm = 0xE9,
    SbcZp = 0xE5,
    SbcZpx = 0xF5,
    SbcAbs = 0xED,
    SbcAbsx = 0xFD,
    SbcAbsy = 0xF9,
    SbcIndx = 0xE1,
    SbcIndy = 0xF1,
    // SEC - Set Carry Flag
    Sec = 0x38,
    // SED - Set Decimal Flag
    Sed = 0xF8,
    // SEI - Set Interrupt Disable Flag
    Sei = 0x78,
    // STA - Store Accumulator
    StaZp = 0x85,
    StaZpx = 0x95,
    StaAbs = 0x8D,
    StaAbsx = 0x9D,
    StaAbsy = 0x99,
    StaIndx = 0x81,
    StaIndy = 0x91,
    // STX - Store X Register
    StxZp = 0x86,
    StxZpy = 0x96,
    StxAbs = 0x8E,
    // STY - Store Y Register
    StyZp = 0x84,
    StyZpx = 0x94,
    StyAbs = 0x8C,
    // Register Transfers
    Tax = 0xAA,
    Tay = 0xA8,
    Tsx = 0xBA,
    Txa = 0x8A,
    Txs = 0x9A,
    Tya = 0x98,
}

impl TryFrom<u8> for Instruction {
    type Error = u8;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x69 => Ok(Self::AdcIm),
            0x65 => Ok(Self::AdcZp),
            0x75 => Ok(Self::AdcZpx),
            0x6D => Ok(Self::AdcAbs),
            0x7D => Ok(Self::AdcAbsx),
            0x79 => Ok(Self::AdcAbsy),
            0x61 => Ok(Self::AdcIndx),
            0x71 => Ok(Self::AdcIndy),
            0x29 => Ok(Self::AndIm),
            0x25 => Ok(Self::AndZp),
            0x35 => Ok(Self::AndZpx),
            0x2D => Ok(Self::AndAbs),
            0x3D => Ok(Self::AndAbsx),
            0x39 => Ok(Self::AndAbsy),
            0x21 => Ok(Self::AndIndx),
            0x31 => Ok(Self::AndIndy),
            0x0A => Ok(Self::AslA),
            0x06 => Ok(Self::AslZp),
            0x16 => Ok(Self::AslZpx),
            0x0E => Ok(Self::AslAbs),
            0x1E => Ok(Self::AslAbsx),
            0x90 => Ok(Self::BccRel),
            0xB0 => Ok(Self::BcsRel),
            0xF0 => Ok(Self::BeqRel),
            0x24 => Ok(Self::BitZp),
            0x2C => Ok(Self::BitAbs),
            0x30 => Ok(Self::BmiRel),
            0xD0 => Ok(Self::BneRel),
            0x10 => Ok(Self::BplRel),
            0x50 => Ok(Self::BvcRel),
            0x70 => Ok(Self::BvsRel),
            0x00 => Ok(Self::Brk),
            0x18 => Ok(Self::Clc),
            0xD8 => Ok(Self::Cld),
            0x58 => Ok(Self::Cli),
            0xB8 => Ok(Self::Clv),
            0xC9 => Ok(Self::CmpIm),
            0xC5 => Ok(Self::CmpZp),
            0xD5 => Ok(Self::CmpZpx),
            0xCD => Ok(Self::CmpAbs),
            0xDD => Ok(Self::CmpAbsx),
            0xD9 => Ok(Self::CmpAbsy),
            0xC1 => Ok(Self::CmpIndx),
            0xD1 => Ok(Self::CmpIndy),
            0xE0 => Ok(Self::CpxIm),
            0xE4 => Ok(Self::CpxZp),
            0xEC => Ok(Self::CpxAbs),
            0xC0 => Ok(Self::CpyIm),
            0xC4 => Ok(Self::CpyZp),
            0xCC => Ok(Self::CpyAbs),
            0xC6 => Ok(Self::DecZp),
            0xD6 => Ok(Self::DecZpx),
            0xCE => Ok(Self::DecAbs),
            0xDE => Ok(Self::DecAbsx),
            0xCA => Ok(Self::Dex),
            0x88 => Ok(Self::Dey),
            0x49 => Ok(Self::EorIm),
            0x45 => Ok(Self::EorZp),
            0x55 => Ok(Self::EorZpx),
            0x4D => Ok(Self::EorAbs),
            0x5D => Ok(Self::EorAbsx),
            0x59 => Ok(Self::EorAbsy),
            0x41 => Ok(Self::EorIndx),
            0x51 => Ok(Self::EorIndy),
            0xE6 => Ok(Self::IncZp),
            0xF6 => Ok(Self::IncZpx),
            0xEE => Ok(Self::IncAbs),
            0xFE => Ok(Self::IncAbsx),
            0xE8 => Ok(Self::Inx),
            0xC8 => Ok(Self::Iny),
            0x4C => Ok(Self::JmpAbs),
            0x6C => Ok(Self::JmpInd),
            0x20 => Ok(Self::Jsr),
            0xA9 => Ok(Self::LdaIm),
            0xA5 => Ok(Self::LdaZp),
            0xB5 => Ok(Self::LdaZpx),
            0xAD => Ok(Self::LdaAbs),
            0xBD => Ok(Self::LdaAbsx),
            0xB9 => Ok(Self::LdaAbsy),
            0xA1 => Ok(Self::LdaIndx),
            0xB1 => Ok(Self::LdaIndy),
            0xA2 => Ok(Self::LdxIm),
            0xA6 => Ok(Self::LdxZp),
            0xB6 => Ok(Self::LdxZpy),
            0xAE => Ok(Self::LdxAbs),
            0xBE => Ok(Self::LdxAbsy),
            0xA0 => Ok(Self::LdyIm),
            0xA4 => Ok(Self::LdyZp),
            0xB4 => Ok(Self::LdyZpx),
            0xAC => Ok(Self::LdyAbs),
            0xBC => Ok(Self::LdyAbsx),
            0x4A => Ok(Self::LsrA),
            0x46 => Ok(Self::LsrZp),
            0x56 => Ok(Self::LsrZpx),
            0x4E => Ok(Self::LsrAbs),
            0x5E => Ok(Self::LsrAbsx),
            0xEA => Ok(Self::Nop),
            0x09 => Ok(Self::OraIm),
            0x05 => Ok(Self::OraZp),
            0x15 => Ok(Self::OraZpx),
            0x0D => Ok(Self::OraAbs),
            0x1D => Ok(Self::OraAbsx),
            0x19 => Ok(Self::OraAbsy),
            0x01 => Ok(Self::OraIndx),
            0x11 => Ok(Self::OraIndy),
            0x48 => Ok(Self::Pha),
            0x08 => Ok(Self::Php),
            0x68 => Ok(Self::Pla),
            0x28 => Ok(Self::Plp),
            0x2A => Ok(Self::RolA),
            0x26 => Ok(Self::RolZp),
            0x36 => Ok(Self::RolZpx),
            0x2E => Ok(Self::RolAbs),
            0x3E => Ok(Self::RolAbsx),
            0x6A => Ok(Self::RorA),
            0x66 => Ok(Self::RorZp),
            0x76 => Ok(Self::RorZpx),
            0x6E => Ok(Self::RorAbs),
            0x7E => Ok(Self::RorAbsx),
            0x40 => Ok(Self::Rti),
            0x60 => Ok(Self::Rts),
            0xE9 => Ok(Self::SbcIm),
            0xE5 => Ok(Self::SbcZp),
            0xF5 => Ok(Self::SbcZpx),
            0xED => Ok(Self::SbcAbs),
            0xFD => Ok(Self::SbcAbsx),
            0xF9 => Ok(Self::SbcAbsy),
            0xE1 => Ok(Self::SbcIndx),
            0xF1 => Ok(Self::SbcIndy),
            0x38 => Ok(Self::Sec),
            0xF8 => Ok(Self::Sed),
            0x78 => Ok(Self::Sei),
            0x85 => Ok(Self::StaZp),
            0x95 => Ok(Self::StaZpx),
            0x8D => Ok(Self::StaAbs),
            0x9D => Ok(Self::StaAbsx),
            0x99 => Ok(Self::StaAbsy),
            0x81 => Ok(Self::StaIndx),
            0x91 => Ok(Self::StaIndy),
            0x86 => Ok(Self::StxZp),
            0x96 => Ok(Self::StxZpy),
            0x8E => Ok(Self::StxAbs),
            0x84 => Ok(Self::StyZp),
            0x94 => Ok(Self::StyZpx),
            0x8C => Ok(Self::StyAbs),
            0xAA => Ok(Self::Tax),
            0xA8 => Ok(Self::Tay),
            0xBA => Ok(Self::Tsx),
            0x8A => Ok(Self::Txa),
            0x9A => Ok(Self::Txs),
            0x98 => Ok(Self::Tya),
            unknown => Err(unknown),
        }
    }
}

impl Instruction {
    pub fn address_mode(&self) -> Option<AddressMode> {
        match self {
            Self::AdcIm
            | Self::AndIm
            | Self::CmpIm
            | Self::CpxIm
            | Self::CpyIm
            | Self::EorIm
            | Self::LdaIm
            | Self::LdxIm
            | Self::LdyIm
            | Self::OraIm
            | Self::SbcIm => Some(AddressMode::Immediate),

            Self::AdcZp
            | Self::AndZp
            | Self::AslZp
            | Self::BitZp
            | Self::CmpZp
            | Self::CpxZp
            | Self::CpyZp
            | Self::DecZp
            | Self::EorZp
            | Self::IncZp
            | Self::LdaZp
            | Self::LdxZp
            | Self::LdyZp
            | Self::LsrZp
            | Self::OraZp
            | Self::RolZp
            | Self::RorZp
            | Self::SbcZp
            | Self::StaZp
            | Self::StxZp
            | Self::StyZp => Some(AddressMode::ZeroPage),

            Self::AdcZpx
            | Self::AndZpx
            | Self::AslZpx
            | Self::CmpZpx
            | Self::DecZpx
            | Self::EorZpx
            | Self::IncZpx
            | Self::LdaZpx
            | Self::LdyZpx
            | Self::LsrZpx
            | Self::OraZpx
            | Self::RolZpx
            | Self::RorZpx
            | Self::SbcZpx
            | Self::StaZpx
            | Self::StyZpx => Some(AddressMode::ZeroPageX),

            Self::LdxZpy | Self::StxZpy => Some(AddressMode::ZeroPageY),

            Self::AdcAbs
            | Self::AndAbs
            | Self::AslAbs
            | Self::BitAbs
            | Self::CmpAbs
            | Self::CpxAbs
            | Self::CpyAbs
            | Self::DecAbs
            | Self::EorAbs
            | Self::IncAbs
            | Self::JmpAbs
            | Self::Jsr
            | Self::LdaAbs
            | Self::LdxAbs
            | Self::LdyAbs
            | Self::LsrAbs
            | Self::OraAbs
            | Self::RolAbs
            | Self::RorAbs
            | Self::SbcAbs
            | Self::StaAbs
            | Self::StxAbs
            | Self::StyAbs => Some(AddressMode::Absolute),

            Self::AdcAbsx
            | Self::AndAbsx
            | Self::AslAbsx
            | Self::CmpAbsx
            | Self::DecAbsx
            | Self::EorAbsx
            | Self::IncAbsx
            | Self::LdaAbsx
            | Self::LdyAbsx
            | Self::LsrAbsx
            | Self::OraAbsx
            | Self::RolAbsx
            | Self::RorAbsx
            | Self::SbcAbsx
            | Self::StaAbsx => Some(AddressMode::AbsoluteX),

            Self::AdcAbsy
            | Self::AndAbsy
            | Self::CmpAbsy
            | Self::EorAbsy
            | Self::LdaAbsy
            | Self::LdxAbsy
            | Self::OraAbsy
            | Self::SbcAbsy
            | Self::StaAbsy => Some(AddressMode::AbsoluteY),

            Self::JmpInd => Some(AddressMode::Indirect),

            Self::AdcIndx
            | Self::AndIndx
            | Self::CmpIndx
            | Self::EorIndx
            | Self::LdaIndx
            | Self::OraIndx
            | Self::SbcIndx
            | Self::StaIndx => Some(AddressMode::IndirectX),

            Self::AdcIndy
            | Self::AndIndy
            | Self::CmpIndy
            | Self::EorIndy
            | Self::LdaIndy
            | Self::OraIndy
            | Self::SbcIndy
            | Self::StaIndy => Some(AddressMode::IndirectY),

            Self::BccRel
            | Self::BcsRel
            | Self::BeqRel
            | Self::BmiRel
            | Self::BneRel
            | Self::BplRel
            | Self::BvcRel
            | Self::BvsRel => Some(AddressMode::Relative),

            Self::AslA | Self::LsrA | Self::RolA | Self::RorA => Some(AddressMode::Accumulator),

            Self::Brk
            | Self::Clc
            | Self::Cld
            | Self::Cli
            | Self::Clv
            | Self::Dex
            | Self::Dey
            | Self::Inx
            | Self::Iny
            | Self::Nop
            | Self::Pha
            | Self::Php
            | Self::Pla
            | Self::Plp
            | Self::Rti
            | Self::Rts
            | Self::Sec
            | Self::Sed
            | Self::Sei
            | Self::Tax
            | Self::Tay
            | Self::Tsx
            | Self::Txa
            | Self::Txs
            | Self::Tya => None,
        }
    }

    pub fn size(&self) -> u8 {
        match self.address_mode() {
            None | Some(AddressMode::Accumulator) => 1,
            Some(AddressMode::Immediate)
            | Some(AddressMode::ZeroPage)
            | Some(AddressMode::ZeroPageX)
            | Some(AddressMode::ZeroPageY)
            | Some(AddressMode::Relative)
            | Some(AddressMode::IndirectX)
            | Some(AddressMode::IndirectY) => 2,
            Some(AddressMode::Absolute)
            | Some(AddressMode::AbsoluteX)
            | Some(AddressMode::AbsoluteY)
            | Some(AddressMode::Indirect) => 3,
        }
    }

    pub fn controls_pc(&self) -> bool {
        matches!(
            self,
            Self::JmpAbs
                | Self::JmpInd
                | Self::Jsr
                | Self::Rts
                | Self::Rti
                | Self::BccRel
                | Self::BcsRel
                | Self::BeqRel
                | Self::BmiRel
                | Self::BneRel
                | Self::BplRel
                | Self::BvcRel
                | Self::BvsRel
        )
    }

    pub fn decode(byte: u8) -> Result<DecodedInstruction, u8> {
        let instruction = Instruction::try_from(byte)?;
        let address_mode = instruction.address_mode();
        let size = instruction.size();
        let controls_pc = instruction.controls_pc();
        Ok(DecodedInstruction {
            instruction,
            address_mode,
            size,
            controls_pc,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        addressing::AddressMode,
        instruction::Instruction::{self, LdaAbs},
    };

    #[test]
    fn check_addr() {
        let inst_byte = 0xA9;

        let inst = Instruction::try_from(inst_byte);

        assert_eq!(inst, Ok(Instruction::LdaIm))
    }

    #[test]
    fn check_false_addr() {
        let inst_byte = 0xFF;
        let inst = Instruction::try_from(inst_byte);

        assert_eq!(inst, Err(inst_byte))
    }

    #[test]
    fn check_brk_eval() {
        let inst_byte = 0x00;
        let inst = Instruction::try_from(inst_byte);

        assert_eq!(inst, Ok(Instruction::Brk))
    }

    #[test]
    fn test_accumulator_size() {
        let inst_byte = 0x0A;
        let inst = Instruction::try_from(inst_byte);
        match inst {
            Ok(i) => assert_eq!(i.size(), 1),
            Err(e) => panic!("unexpected invalid opcode: {:#04x}", e),
        }
    }

    #[test]
    fn test_two_byte_size() {
        let opcodes = [0xA9, 0xA5, 0xB5, 0xB6, 0x90, 0xA1, 0xB1];
        for byte in opcodes {
            match Instruction::try_from(byte) {
                Ok(i) => assert_eq!(i.size(), 2, "failed for opcode {:#04x}", byte),
                Err(e) => panic!("unexpected invalid opcode: {:#04x}", e),
            }
        }
    }

    #[test]
    fn test_three_byte_size() {
        let opcodes = [0xAD, 0xBD, 0xB9, 0x6C];
        for byte in opcodes {
            match Instruction::try_from(byte) {
                Ok(i) => assert_eq!(i.size(), 3, "failed for opcode {:#04x}", byte),
                Err(e) => panic!("unexpected invalid opcode: {:#04x}", e),
            }
        }
    }

    #[test]
    fn controls_pc() {
        assert!(Instruction::JmpAbs.controls_pc());
        assert!(!Instruction::LdaAbs.controls_pc());
    }

    #[test]
    fn decode() {
        let inst = 0xA9;
        match Instruction::decode(inst) {
            Ok(i) => {
                assert_eq!(i.instruction, Instruction::LdaIm);
                assert_eq!(i.size, 2);
                assert_eq!(i.address_mode, Some(AddressMode::Immediate));
                assert_eq!(i.controls_pc, false);
            }
            Err(e) => panic!("unexpected invalid opcode: {:#04x}", e),
        }
    }
}

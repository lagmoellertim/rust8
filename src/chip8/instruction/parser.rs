use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;
use thiserror::Error;

use super::Instruction;
use crate::chip8::data_register::DataRegister;

#[derive(Error, Debug)]
pub enum InstructionParsingError {
    #[error("invalid data register '{0:#01x}'")]
    InvalidDataRegister(u8),

    #[error("invalid instruction '{0:#04x}'")]
    InvalidInstruction(u16),
}

struct InstructionParser<'a> {
    raw_data: &'a BitSlice<u8, Msb0>,
}

impl<'a> From<&'a [u8; 2]> for InstructionParser<'a> {
    fn from(value: &'a [u8; 2]) -> Self {
        InstructionParser {
            raw_data: value.view_bits(),
        }
    }
}

impl<'a> InstructionParser<'a> {
    fn nibbles(&self) -> [u8; 4] {
        [
            self.raw_data[0..4].load_be(),
            self.raw_data[4..8].load_be(),
            self.raw_data[8..12].load_be(),
            self.raw_data[12..16].load_be(),
        ]
    }

    fn instruction(&self) -> u16 {
        self.raw_data.load_be()
    }

    fn address(&self) -> usize {
        self.raw_data[4..].load_be()
    }

    fn vx(&self) -> Result<DataRegister, InstructionParsingError> {
        let data_register_index = self.raw_data[4..8].load_be::<u8>();

        data_register_index
            .try_into()
            .map_err(|_| InstructionParsingError::InvalidDataRegister(data_register_index))
    }

    fn vy(&self) -> Result<DataRegister, InstructionParsingError> {
        let data_register_index = self.raw_data[8..12].load_be::<u8>();

        data_register_index
            .try_into()
            .map_err(|_| InstructionParsingError::InvalidDataRegister(data_register_index))
    }

    fn num_8bit(&self) -> u8 {
        self.raw_data[8..16].load_be()
    }

    fn num_4bit(&self) -> u8 {
        self.raw_data[12..16].load_be()
    }

    fn parse_instruction(&self) -> Result<Instruction, InstructionParsingError> {
        match self.nibbles() {
            [0x0, 0x0, 0xE, 0x0] => Ok(Instruction::ClearScreen),
            [0x0, 0x0, 0xE, 0xE] => Ok(Instruction::ReturnFromSubroutine),
            [0x0, _, _, _] => Ok(Instruction::ExecuteMachineLanguageSubroutine {
                address: self.address(),
            }),

            [0x1, _, _, _] => Ok(Instruction::JumpToAddress {
                address: self.address(),
            }),
            [0x2, _, _, _] => Ok(Instruction::ExecuteSubroutine {
                address: self.address(),
            }),
            [0x3, _, _, _] => Ok(Instruction::SkipIfVxEqualsNum {
                vx: self.vx()?,
                num: self.num_8bit(),
            }),
            [0x4, _, _, _] => Ok(Instruction::SkipIfVxNotEqualNum {
                vx: self.vx()?,
                num: self.num_8bit(),
            }),
            [0x5, _, _, 0x0] => Ok(Instruction::SkipIfVxEqualsVy {
                vx: self.vx()?,
                vy: self.vy()?,
            }),
            [0x6, _, _, _] => Ok(Instruction::StoreNumInVx {
                vx: self.vx()?,
                num: self.num_8bit(),
            }),

            [0x7, _, _, _] => Ok(Instruction::AddNumToVx {
                vx: self.vx()?,
                num: self.num_8bit(),
            }),

            [0x8, _, _, 0x0] => Ok(Instruction::StoreVyInVx {
                vx: self.vx()?,
                vy: self.vy()?,
            }),

            [0x8, _, _, 0x1] => Ok(Instruction::SetVxToVxOrVy {
                vx: self.vx()?,
                vy: self.vy()?,
            }),

            [0x8, _, _, 0x2] => Ok(Instruction::SetVxToVxAndVy {
                vx: self.vx()?,
                vy: self.vy()?,
            }),
            [0x8, _, _, 0x3] => Ok(Instruction::SetVxToVxXorVy {
                vx: self.vx()?,
                vy: self.vy()?,
            }),
            [0x8, _, _, 0x4] => Ok(Instruction::AddVyToVx {
                vx: self.vx()?,
                vy: self.vy()?,
            }),
            [0x8, _, _, 0x5] => Ok(Instruction::SubtractVyFromVx {
                vx: self.vx()?,
                vy: self.vy()?,
            }),
            [0x8, _, _, 0x6] => Ok(Instruction::ShiftVyRightStoreInVx {
                vx: self.vx()?,
                vy: self.vy()?,
            }),
            [0x8, _, _, 0x7] => Ok(Instruction::SetVxToVyMinusVx {
                vx: self.vx()?,
                vy: self.vy()?,
            }),
            [0x8, _, _, 0xE] => Ok(Instruction::ShiftVyLeftStoreInVx {
                vx: self.vx()?,
                vy: self.vy()?,
            }),
            [0x9, _, _, 0x0] => Ok(Instruction::SkipIfVxNotEqualVy {
                vx: self.vx()?,
                vy: self.vy()?,
            }),
            [0xA, _, _, _] => Ok(Instruction::StoreAddressInAddressRegister {
                address: self.address(),
            }),
            [0xB, _, _, _] => Ok(Instruction::JumpToAddressPlusV0 {
                address: self.address(),
            }),
            [0xC, _, _, _] => Ok(Instruction::SetVxToRandomWithMask {
                vx: self.vx()?,
                mask: self.num_8bit(),
            }),
            [0xD, _, _, _] => Ok(Instruction::DrawSpriteAtVxVy {
                vx: self.vx()?,
                vy: self.vy()?,
                byte_count: self.num_4bit(),
            }),
            [0xE, _, 0x9, 0xE] => Ok(Instruction::SkipIfKeyInVxPressed { vx: self.vx()? }),
            [0xE, _, 0xA, 0x1] => Ok(Instruction::SkipIfKeyInVxNotPressed { vx: self.vx()? }),
            [0xF, _, 0x0, 0x7] => Ok(Instruction::StoreDelayTimerInVx { vx: self.vx()? }),
            [0xF, _, 0x0, 0xA] => Ok(Instruction::WaitForKeypressStoreInVx { vx: self.vx()? }),
            [0xF, _, 0x1, 0x5] => Ok(Instruction::SetDelayTimerToVx { vx: self.vx()? }),
            [0xF, _, 0x1, 0x8] => Ok(Instruction::SetSoundTimerToVx { vx: self.vx()? }),
            [0xF, _, 0x1, 0xE] => Ok(Instruction::AddVxToAddressRegister { vx: self.vx()? }),
            [0xF, _, 0x2, 0x9] => {
                Ok(Instruction::SetAddressRegisterToSpriteAddressOfSpriteInVx { vx: self.vx()? })
            }
            [0xF, _, 0x3, 0x3] => Ok(Instruction::StoreBCDOfVx { vx: self.vx()? }),
            [0xF, _, 0x5, 0x5] => Ok(Instruction::StoreRegistersInMemory { vx: self.vx()? }),
            [0xF, _, 0x6, 0x5] => Ok(Instruction::FillRegistersFromMemory { vx: self.vx()? }),
            _ => Err(InstructionParsingError::InvalidInstruction(
                self.instruction(),
            )),
        }
    }
}

impl TryFrom<&[u8; 2]> for Instruction {
    type Error = InstructionParsingError;

    fn try_from(value: &[u8; 2]) -> Result<Self, Self::Error> {
        InstructionParser::from(value).parse_instruction()
    }
}

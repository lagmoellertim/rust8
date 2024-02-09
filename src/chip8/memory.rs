use thiserror::Error;

use super::constants::{INSTRUCTION_SIZE, MEMORY_SIZE, UNPROTECTED_MEMORY_START};
use super::graphic::BitSlicePixelView;
use super::instruction::Instruction;
use crate::chip8::instruction::parser::InstructionParsingError;

pub struct Memory {
    pub raw_data: [u8; MEMORY_SIZE],
}

#[derive(Error, Debug)]
pub enum ReadInstructionError {
    #[error("instruction parsing error")]
    ParseError(#[from] InstructionParsingError),
    #[error("instruction address in protected memory area")]
    AddressInProtectedMemoryArea(usize),
    #[error("instruction address outside of the memory")]
    AddressOutOfRange(usize),
}

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("address range is inside of protected memory area")]
    InProtectedMemoryArea(usize, usize),
    #[error("address range is outside of the memory")]
    OutOfRange(usize, usize),
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            raw_data: [0; MEMORY_SIZE],
        }
    }
}

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.raw_data.fill(0);
    }

    pub fn write_unrestricted(
        &mut self,
        data: &[u8],
        write_address: usize,
    ) -> Result<(), WriteError> {
        self.raw_data
            .get_mut(write_address..write_address + data.len())
            .ok_or(WriteError::OutOfRange(
                write_address,
                write_address + data.len(),
            ))?
            .copy_from_slice(data);

        Ok(())
    }

    pub fn write_restricted(
        &mut self,
        data: &[u8],
        write_address: usize,
    ) -> Result<(), WriteError> {
        if write_address < UNPROTECTED_MEMORY_START {
            return Err(WriteError::InProtectedMemoryArea(
                write_address,
                write_address + data.len(),
            ));
        }

        self.write_unrestricted(data, write_address)
    }

    pub fn read_instruction(&self, address: usize) -> Result<Instruction, ReadInstructionError> {
        if address < UNPROTECTED_MEMORY_START {
            return Err(ReadInstructionError::AddressInProtectedMemoryArea(address));
        }

        let instruction_slice: &[u8; INSTRUCTION_SIZE] = self
            .raw_data
            .get(address..address + INSTRUCTION_SIZE)
            .ok_or(ReadInstructionError::AddressOutOfRange(address))?
            .try_into()
            .unwrap();

        Ok(Instruction::try_from(instruction_slice)?)
    }

    pub fn read_sprite(&self, address: usize, byte_count: usize) -> Option<BitSlicePixelView> {
        let slice = self.raw_data.get(address..address + byte_count)?;

        Some(BitSlicePixelView::new_from_byte_slice(slice, 8, byte_count))
    }
}

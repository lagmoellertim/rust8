use thiserror::Error;
use crate::Chip8;
use crate::chip8::Blocked;
use crate::constants::{FONT_SPRITE_MEMORY_LOCATION, FONT_SPRITE_SIZE, INSTRUCTION_SIZE};
use crate::data_register::DataRegister;
use crate::graphic::{PixelView, XorPixelErased};
use crate::instruction::Instruction;
use crate::keyboard::{Key, KeyState};


#[derive(Error, Debug)]
pub enum InstructionExecutionError {
    #[error("invalid return, no address on stack to jump back to")]
    InvalidReturn,
    #[error("invalid memory access, attempted to access {0:#04x}")]
    InvalidMemoryAccess(usize),
    #[error("invalid key {0:#01x} specified ")]
    InvalidKey(u8),
}

impl Chip8 {
    pub fn execute_instruction(
        &mut self,
        instruction: Instruction,
    ) -> Result<(), InstructionExecutionError> {
        match instruction {
            Instruction::ExecuteMachineLanguageSubroutine { .. } => {
                // This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.
                Ok(())
            }
            Instruction::ClearScreen => {
                self.screen.clear();

                Ok(())
            }
            Instruction::ReturnFromSubroutine => {
                //The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
                self.program_counter = self
                    .stack
                    .pop()
                    .ok_or(InstructionExecutionError::InvalidReturn)?;
                self.in_jump = true;

                Ok(())
            }
            Instruction::JumpToAddress { address } => {
                self.program_counter = address;
                self.in_jump = true;

                Ok(())
            }

            Instruction::ExecuteSubroutine { address } => {
                // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
                self.stack.push(self.program_counter + INSTRUCTION_SIZE);
                self.program_counter = address;
                self.in_jump = true;

                Ok(())
            }

            Instruction::SkipIfVxEqualsNum { vx, num } => {
                if self.data_registers[vx] == num {
                    self.program_counter += INSTRUCTION_SIZE;
                }

                Ok(())
            }

            Instruction::SkipIfVxNotEqualNum { vx, num } => {
                if self.data_registers[vx] != num {
                    self.program_counter += INSTRUCTION_SIZE;
                }

                Ok(())
            }

            Instruction::SkipIfVxEqualsVy { vx, vy } => {
                if self.data_registers[vx] == self.data_registers[vy] {
                    self.program_counter += INSTRUCTION_SIZE;
                }

                Ok(())
            }

            Instruction::StoreNumInVx { vx, num } => {
                self.data_registers[vx] = num;

                Ok(())
            }

            Instruction::AddNumToVx { vx, num } => {
                self.data_registers[vx] += num;

                Ok(())
            }

            Instruction::StoreVyInVx { vx, vy } => {
                self.data_registers[vx] = self.data_registers[vy];

                Ok(())
            }

            Instruction::SetVxToVxOrVy { vx, vy } => {
                self.data_registers[vx] = self.data_registers[vx] | self.data_registers[vy];

                Ok(())
            }

            Instruction::SetVxToVxAndVy { vx, vy } => {
                self.data_registers[vx] = self.data_registers[vx] & self.data_registers[vy];

                Ok(())
            }

            Instruction::SetVxToVxXorVy { vx, vy } => {
                self.data_registers[vx] = self.data_registers[vx] ^ self.data_registers[vy];

                Ok(())
            }

            Instruction::AddVyToVx { vx, vy } => {
                // Add the value of register vy to register vx
                let (value, overflow) =
                    self.data_registers[vx].overflowing_add(self.data_registers[vy]);

                self.data_registers[vx] = value;

                self.data_registers[DataRegister::VF] = match overflow {
                    true => 1,  // Set REG_F to 1 if a carry occurs
                    false => 0, // Set REG_F to 0 if a carry does not occur
                };

                Ok(())
            }

            Instruction::SubtractVyFromVx { vx, vy } => {
                let (value, overflow) =
                    self.data_registers[vx].overflowing_sub(self.data_registers[vy]);
                self.data_registers[vx] = value;
                self.data_registers[DataRegister::VF] = match overflow {
                    true => 0,  // Set REG_F to 0 if a borrow occurs
                    false => 1, // Set REG_F to 1 if a borrow does not occur
                };

                Ok(())
            }

            Instruction::ShiftVyRightStoreInVx { vx, vy } => {
                // Set register REG_F to the least significant bit prior to the shift
                self.data_registers[DataRegister::VF] = self.data_registers[vy] & 1;

                // Store the value of register vy shifted right one bit in register vx
                self.data_registers[vx] = self.data_registers[vy] >> 1;

                Ok(())
            }

            Instruction::SetVxToVyMinusVx { vx, vy } => {
                let (value, overflow) =
                    self.data_registers[vy].overflowing_sub(self.data_registers[vx]);
                self.data_registers[vx] = value;

                self.data_registers[DataRegister::VF] = match overflow {
                    true => 0,  // Set REG_F to 0 if a borrow occurs
                    false => 1, // Set REG_F to 1 if a borrow does not occur
                };

                Ok(())
            }

            Instruction::ShiftVyLeftStoreInVx { vx, vy } => {
                // Set register REG_F to the most significant bit prior to the shift
                self.data_registers[DataRegister::VF] = self.data_registers[vy] >> 7;

                // Store the value of register vy shifted left one bit in register vx
                self.data_registers[vx] = self.data_registers[vy] << 1;

                Ok(())
            }

            Instruction::SkipIfVxNotEqualVy { vx, vy } => {
                // Skip the following instruction if the value of register VX is not equal to the value of register VY
                if self.data_registers[vx] != self.data_registers[vy] {
                    self.program_counter += INSTRUCTION_SIZE;
                }

                Ok(())
            }

            Instruction::StoreAddressInAddressRegister { address } => {
                // Store memory address NNN in register Address Register
                self.address_register = address as usize;

                Ok(())
            }

            Instruction::JumpToAddressPlusV0 { address } => {
                // Jump to address NNN + V0
                self.program_counter = address + self.data_registers[DataRegister::V0] as usize;
                self.in_jump = true;

                Ok(())
            }

            Instruction::SetVxToRandomWithMask { vx, mask } => {
                self.data_registers[vx] = rand::random::<u8>() & mask;

                Ok(())
            }

            Instruction::DrawSpriteAtVxVy { vx, vy, byte_count } => {
                // The interpreter reads n bytes from memory, starting at the address stored in I.
                // These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).

                let sprite = self
                    .memory
                    .read_sprite(self.address_register, byte_count as usize)
                    .ok_or_else(|| {
                        InstructionExecutionError::InvalidMemoryAccess(
                            self.address_register + byte_count as usize,
                        )
                    })?;

                let sprite_x_pos = self.data_registers[vx] as usize;
                let sprite_y_pos = self.data_registers[vy] as usize;

                let mut pixel_erased = XorPixelErased::No;

                for y in 0..sprite.height() {
                    for x in 0..sprite.width() {
                        let current_pixel_erased = self.screen.xor_pixel_wrapped_position(
                            sprite_x_pos + x,
                            sprite_y_pos + y,
                            sprite.get_pixel_unchecked(x, y),
                        );

                        if let XorPixelErased::Yes = current_pixel_erased {
                            pixel_erased = XorPixelErased::Yes;
                        }
                    }
                }

                self.data_registers[DataRegister::VF] = match pixel_erased {
                    XorPixelErased::Yes => 1, // If this causes any pixels to be erased, VF is set to 1
                    XorPixelErased::No => 0,  // Otherwise it is set to 0
                };

                Ok(())
            }

            Instruction::SkipIfKeyInVxPressed { vx } => {
                // Skip next instruction if key with the value of Vx is pressed.
                if let KeyState::Pressed =
                    self.keyboard
                        .get_key_state(Key::try_from(self.data_registers[vx]).map_err(|_| {
                            InstructionExecutionError::InvalidKey(self.data_registers[vx])
                        })?)
                {
                    self.program_counter += INSTRUCTION_SIZE;
                }

                Ok(())
            }

            Instruction::SkipIfKeyInVxNotPressed { vx } => {
                // Skip next instruction if key with the value of Vx is not pressed.
                if let KeyState::Released =
                    self.keyboard
                        .get_key_state(Key::try_from(self.data_registers[vx]).map_err(|_| {
                            InstructionExecutionError::InvalidKey(self.data_registers[vx])
                        })?)
                {
                    self.program_counter += INSTRUCTION_SIZE;
                }

                Ok(())
            }

            Instruction::StoreDelayTimerInVx { vx } => {
                // Store the current value of the delay timer in register VX
                self.data_registers[vx] = self.delay_timer;

                Ok(())
            }

            Instruction::WaitForKeypressStoreInVx { vx } => {
                self.blocked = Blocked::WaitingOnKeyUp(vx);

                Ok(())
            },
            Instruction::SetDelayTimerToVx { vx } => {
                self.delay_timer = self.data_registers[vx];

                Ok(())
            }

            Instruction::SetSoundTimerToVx { vx } => {
                // Set the sound timer to the value of register VX
                self.sound_timer = self.data_registers[vx];

                Ok(())
            }

            Instruction::AddVxToAddressRegister { vx } => {
                self.address_register += self.data_registers[vx] as usize;

                Ok(())
            }

            Instruction::SetAddressRegisterToSpriteAddressOfSpriteInVx { vx } => {
                // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
                self.address_register = FONT_SPRITE_MEMORY_LOCATION
                    + (self.data_registers[vx] as usize * FONT_SPRITE_SIZE);

                Ok(())
            }

            Instruction::StoreBCDOfVx { vx } => {
                // Store the binary-coded decimal equivalent of the value stored in register VX at addresses I, I + 1, and I + 2
                let num = self.data_registers[vx];

                if self.address_register + 2 >= self.memory.raw_data.len() {
                    return Err(InstructionExecutionError::InvalidMemoryAccess(
                        self.address_register + 2,
                    ));
                }

                // Hundreds digit in memory at location in I
                self.memory.raw_data[self.address_register] = num / 100;

                // Tens digit at location I+1
                self.memory.raw_data[self.address_register + 1] = (num % 100) / 10;

                // Ones digit at location I+2
                self.memory.raw_data[self.address_register + 2] = num % 10;

                Ok(())
            }

            Instruction::StoreRegistersInMemory { vx } => {
                // Store the values of registers V0 to VX inclusive in memory starting at address I
                for register_num in 0..=vx.into() {
                    let memory_address = self.address_register + register_num as usize;
                    let memory_ref =
                        self.memory
                            .raw_data
                            .get_mut(memory_address)
                            .ok_or_else(|| {
                                InstructionExecutionError::InvalidMemoryAccess(memory_address)
                            })?;

                    *memory_ref = self.data_registers[register_num.try_into().unwrap()];
                }

                Ok(())
            }

            Instruction::FillRegistersFromMemory { vx } => {
                // Fill registers V0 to VX inclusive with the values stored in memory starting at address I
                // Store the values of registers V0 to VX inclusive in memory starting at address I
                for register_num in 0..=vx.into() {
                    let memory_address = self.address_register + register_num as usize;

                    self.data_registers[register_num.try_into().unwrap()] =
                        *self.memory.raw_data.get(memory_address).ok_or_else(|| {
                            InstructionExecutionError::InvalidMemoryAccess(memory_address)
                        })?
                }

                Ok(())
            }
        }
    }
}

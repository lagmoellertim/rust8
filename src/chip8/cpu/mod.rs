pub mod execute;
use thiserror::Error;

use self::execute::InstructionExecutionError;
use super::constants::INSTRUCTION_SIZE;
use super::{Blocked, Chip8};
use crate::chip8::Key;
use crate::memory::ReadInstructionError;

#[derive(Error, Debug)]
pub enum CycleError {
    #[error("instruction fetch error")]
    FetchError(#[from] ReadInstructionError),
    #[error("instruction execution error")]
    ExecutionError(#[from] InstructionExecutionError),
}

impl Chip8 {
    pub fn cycle(&mut self) -> Result<(), CycleError> {
        if self.blocked != Blocked::No {
            return Ok(());
        }

        let instruction = self.memory.read_instruction(self.program_counter)?;
        self.execute_instruction(instruction)?;

        // Auto-Increment only when not in jump
        if !self.in_jump {
            self.program_counter += INSTRUCTION_SIZE;
        }
        self.in_jump = false;

        Ok(())
    }

    pub fn handle_key_up_interrupt(&mut self, key: Key) {
        if let Blocked::WaitingOnKeyUp(vx) = self.blocked {
            self.data_registers[vx] = u8::from(key);
            self.blocked = Blocked::No;
        }
    }

    pub fn update_timers(&mut self) {
        if self.blocked != Blocked::No {
            return;
        }

        // Should be called at a rate of 60hz
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}

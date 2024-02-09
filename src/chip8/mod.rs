use self::constants::{DEFAULT_PROGRAM_ADDRESS, FONT_SPRITES, FONT_SPRITE_SIZE, STACK_SIZE};
use self::data_register::{DataRegister, DataRegisters};
use self::graphic::Screen;
use self::keyboard::{Key, Keyboard};
use self::memory::{Memory, WriteError};

pub mod constants;
pub mod cpu;
pub mod data_register;
pub mod graphic;
pub mod instruction;
pub mod keyboard;
pub mod memory;

#[derive(PartialEq)]
enum Blocked {
    No,
    WaitingOnKeyUp(DataRegister),
}

pub struct Chip8 {
    pub memory: Memory,

    pub data_registers: DataRegisters,
    pub address_register: usize,
    pub sound_timer: u8,
    pub delay_timer: u8,
    pub program_counter: usize,

    keyboard: Keyboard,
    pub stack: Vec<usize>,
    pub screen: Screen,
    in_jump: bool,
    blocked: Blocked,
}

impl Default for Chip8 {
    fn default() -> Self {
        Chip8 {
            data_registers: DataRegisters::new(),
            address_register: 0,
            keyboard: Keyboard::new(),
            in_jump: false,
            blocked: Blocked::No,
            delay_timer: 0,
            sound_timer: 0,
            stack: Vec::with_capacity(STACK_SIZE),
            program_counter: DEFAULT_PROGRAM_ADDRESS,
            memory: Memory::new(),
            screen: Screen::new(),
        }
    }
}

impl Chip8 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.data_registers.reset();
        self.address_register = 0;
        self.in_jump = false;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.stack.clear();
        self.program_counter = DEFAULT_PROGRAM_ADDRESS;
        self.memory.clear();
        self.screen.clear();
        self.blocked = Blocked::No;
    }

    pub fn key_up(&mut self, key: Key) {
        self.keyboard.key_up(key);
        self.handle_key_up_interrupt(key)
    }

    pub fn key_down(&mut self, key: Key) {
        self.keyboard.key_down(key);
    }

    fn load_font_sprites(&mut self) {
        for (i, sprite) in FONT_SPRITES.iter().enumerate() {
            self.memory
                .write_unrestricted(sprite, i * FONT_SPRITE_SIZE)
                .unwrap();
        }
    }

    pub fn load_program_to_address(
        &mut self,
        program: &[u8],
        address: usize,
    ) -> Result<(), WriteError> {
        self.reset();
        self.load_font_sprites();

        self.memory.write_restricted(program, address)?;
        self.program_counter = address;

        Ok(())
    }

    pub fn load_program(&mut self, program: &[u8]) -> Result<(), WriteError> {
        self.load_program_to_address(program, DEFAULT_PROGRAM_ADDRESS)
    }

    pub fn is_blocked(&self) -> bool {
        self.blocked != Blocked::No
    }
}

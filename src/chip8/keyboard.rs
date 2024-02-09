use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Key {
    Num0 = 0x0,
    Num1 = 0x1,
    Num2 = 0x2,
    Num3 = 0x3,
    Num4 = 0x4,
    Num5 = 0x5,
    Num6 = 0x6,
    Num7 = 0x7,
    Num8 = 0x8,
    Num9 = 0x9,
    A = 0xA,
    B = 0xB,
    C = 0xC,
    D = 0xD,
    E = 0xE,
    F = 0xF,
}

#[derive(Clone, Copy)]
pub enum KeyState {
    Pressed,
    Released,
}

pub struct Keyboard {
    raw: [KeyState; 16],
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard {
            raw: [KeyState::Released; 16],
        }
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn key_down(&mut self, key: Key) {
        self.raw[u8::from(key) as usize] = KeyState::Pressed;
    }

    pub fn key_up(&mut self, key: Key) {
        self.raw[u8::from(key) as usize] = KeyState::Released;
    }

    pub fn get_key_state(&self, key: Key) -> KeyState {
        self.raw[u8::from(key) as usize]
    }
}

impl TryFrom<char> for Key {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            '0' => Ok(Key::Num0),
            '1' => Ok(Key::Num1),
            '2' => Ok(Key::Num2),
            '3' => Ok(Key::Num3),
            '4' => Ok(Key::Num4),
            '5' => Ok(Key::Num5),
            '6' => Ok(Key::Num6),
            '7' => Ok(Key::Num7),
            '8' => Ok(Key::Num8),
            '9' => Ok(Key::Num9),
            'a' => Ok(Key::A),
            'b' => Ok(Key::B),
            'c' => Ok(Key::C),
            'd' => Ok(Key::D),
            'e' => Ok(Key::E),
            'f' => Ok(Key::F),
            _ => Err(()),
        }
    }
}

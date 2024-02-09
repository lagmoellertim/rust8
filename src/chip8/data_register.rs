use std::fmt::{self, Display, Formatter};
use std::ops::{Index, IndexMut};

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum DataRegister {
    V0 = 0x0,
    V1 = 0x1,
    V2 = 0x2,
    V3 = 0x3,
    V4 = 0x4,
    V5 = 0x5,
    V6 = 0x6,
    V7 = 0x7,
    V8 = 0x8,
    V9 = 0x9,
    VA = 0xA,
    VB = 0xB,
    VC = 0xC,
    VD = 0xD,
    VE = 0xE,
    VF = 0xF,
}

#[derive(Default)]
pub struct DataRegisters {
    data: [u8; 16],
}

impl DataRegisters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.data = [0; 16]
    }
}

impl Index<DataRegister> for DataRegisters {
    type Output = u8;

    fn index(&self, index: DataRegister) -> &Self::Output {
        &self.data[u8::from(index) as usize]
    }
}

impl IndexMut<DataRegister> for DataRegisters {
    fn index_mut(&mut self, index: DataRegister) -> &mut Self::Output {
        &mut self.data[u8::from(index) as usize]
    }
}

impl Display for DataRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "v{:#01X}", u8::from(*self))
    }
}

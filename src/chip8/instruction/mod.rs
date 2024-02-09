#[derive(Debug)]
pub enum Instruction {
    ExecuteMachineLanguageSubroutine {
        address: usize,
    },
    ClearScreen,
    ReturnFromSubroutine,
    JumpToAddress {
        address: usize,
    },
    ExecuteSubroutine {
        address: usize,
    },
    SkipIfVxEqualsNum {
        vx: DataRegister,
        num: u8,
    },
    SkipIfVxNotEqualNum {
        vx: DataRegister,
        num: u8,
    },
    SkipIfVxEqualsVy {
        vx: DataRegister,
        vy: DataRegister,
    },
    StoreNumInVx {
        vx: DataRegister,
        num: u8,
    },
    AddNumToVx {
        vx: DataRegister,
        num: u8,
    },
    StoreVyInVx {
        vx: DataRegister,
        vy: DataRegister,
    },
    SetVxToVxOrVy {
        vx: DataRegister,
        vy: DataRegister,
    },
    SetVxToVxAndVy {
        vx: DataRegister,
        vy: DataRegister,
    },
    SetVxToVxXorVy {
        vx: DataRegister,
        vy: DataRegister,
    },
    AddVyToVx {
        vx: DataRegister,
        vy: DataRegister,
    },
    SubtractVyFromVx {
        vx: DataRegister,
        vy: DataRegister,
    },
    ShiftVyRightStoreInVx {
        vx: DataRegister,
        vy: DataRegister,
    },
    SetVxToVyMinusVx {
        vx: DataRegister,
        vy: DataRegister,
    },
    ShiftVyLeftStoreInVx {
        vx: DataRegister,
        vy: DataRegister,
    },
    SkipIfVxNotEqualVy {
        vx: DataRegister,
        vy: DataRegister,
    },
    StoreAddressInAddressRegister {
        address: usize,
    },
    JumpToAddressPlusV0 {
        address: usize,
    },
    SetVxToRandomWithMask {
        vx: DataRegister,
        mask: u8,
    },
    DrawSpriteAtVxVy {
        vx: DataRegister,
        vy: DataRegister,
        byte_count: u8,
    },
    SkipIfKeyInVxPressed {
        vx: DataRegister,
    },
    SkipIfKeyInVxNotPressed {
        vx: DataRegister,
    },
    StoreDelayTimerInVx {
        vx: DataRegister,
    },
    WaitForKeypressStoreInVx {
        vx: DataRegister,
    },
    SetDelayTimerToVx {
        vx: DataRegister,
    },
    SetSoundTimerToVx {
        vx: DataRegister,
    },
    AddVxToAddressRegister {
        vx: DataRegister,
    },
    SetAddressRegisterToSpriteAddressOfSpriteInVx {
        vx: DataRegister,
    },
    StoreBCDOfVx {
        vx: DataRegister,
    },
    StoreRegistersInMemory {
        vx: DataRegister,
    },
    FillRegistersFromMemory {
        vx: DataRegister,
    },
}

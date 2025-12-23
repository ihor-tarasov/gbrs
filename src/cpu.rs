pub struct Cpu {
    pub pc: u16,
    pub sp: u16,
}

impl Cpu {
    pub const fn new() -> Self {
        Self { pc: 0, sp: 0 }
    }
}

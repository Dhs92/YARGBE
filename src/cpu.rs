use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Default)]
    pub struct Flags: u8 {
        const ZERO = 0b0000_0001;
        const SUBTRACT = 0b0000_0010;
        const HALFCARRY = 0b0000_0100;
        const CARRY = 0b0000_1000;
    }
}

#[derive(Debug, Default)]
pub struct Registers {
    a: u8,
    f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8
}

#[derive(Debug, Default)]
pub struct CPU {
    registers: Registers,

    sp: u16,
    pc: u16,
    interrupt_flag_register: u8,
    interrupt_enable_register: u8,
    interrupt_master_enable: bool,
    interrupt_queued: bool,
    
    halted: bool,
    stopped: bool,
    cycles: u8,
}

impl Registers {
    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | self.f.bits() as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = Flags::from_bits_truncate((value & 0xFF) as u8);
    }
    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

impl CPU {
    pub fn init() -> Self {
        Self::default()
    }
}
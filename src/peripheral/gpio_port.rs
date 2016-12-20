# [ doc = "GPIO port" ]
# [ repr ( C ) ]
pub struct GpioPort {
    # [ doc = "0x00 - Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31" ]
    pub b: [B; 256],
    _reserved0: [u8; 3840usize],
    # [ doc = "0x1000 - Word pin registers port 0 to 5" ]
    pub w: [W; 256],
    _reserved1: [u8; 3072usize],
    # [ doc = "0x2000 - Direction registers port m" ]
    pub dir: [Dir; 8],
    _reserved2: [u8; 96usize],
    # [ doc = "0x2080 - Mask register port m" ]
    pub mask: [Mask; 8],
    _reserved3: [u8; 96usize],
    # [ doc = "0x2100 - Port pin register port m" ]
    pub pin: [Pin; 8],
    _reserved4: [u8; 96usize],
    # [ doc = "0x2180 - Masked port register port m" ]
    pub mpin: [Mpin; 8],
    _reserved5: [u8; 96usize],
    # [ doc = "0x2200 - Write: Set register for port m Read: output bits for port m" ]
    pub set: [Set; 8],
    _reserved6: [u8; 96usize],
    # [ doc = "0x2280 - Clear port m" ]
    pub clr: [Clr; 8],
    _reserved7: [u8; 96usize],
    # [ doc = "0x2300 - Toggle port m" ]
    pub not: [Not; 8],
}

# [ repr ( C ) ]
pub struct B {
    register: ::volatile_register::RW<u8>,
}

impl B {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BR, &'w mut BW) -> &'w mut BW
    {
        let bits = self.register.read();
        let r = BR { bits: bits };
        let mut w = BW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BR {
        BR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BW) -> &mut BW
    {
        let mut w = BW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BR {
    bits: u8,
}

impl BR {
    # [ doc = "Bit 0 - Read: state of the GPIOm[n] pin, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. Write: loads the pin's output bit." ]
    pub fn pbyte(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BW {
    bits: u8,
}

impl BW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BW { bits: 0 }
    }
    # [ doc = "Bit 0 - Read: state of the GPIOm[n] pin, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. Write: loads the pin's output bit." ]
    pub fn pbyte(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct W {
    register: ::volatile_register::RW<u32>,
}

impl W {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&WR, &'w mut WW) -> &'w mut WW
    {
        let bits = self.register.read();
        let r = WR { bits: bits };
        let mut w = WW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> WR {
        WR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut WW) -> &mut WW
    {
        let mut w = WW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct WR {
    bits: u32,
}

impl WR {
    # [ doc = "Bits 0:31 - Read 0: pin is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit." ]
    pub fn pword(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct WW {
    bits: u32,
}

impl WW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        WW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - Read 0: pin is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit." ]
    pub fn pword(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dir {
    register: ::volatile_register::RW<u32>,
}

impl Dir {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DirR, &'w mut DirW) -> &'w mut DirW
    {
        let bits = self.register.read();
        let r = DirR { bits: bits };
        let mut w = DirW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DirR {
        DirR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DirW) -> &mut DirW
    {
        let mut w = DirW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DirR {
    bits: u32,
}

impl DirR {
    # [ doc = "Bit 0 - Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp16(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp17(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp18(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp19(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp20(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp21(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp22(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp23(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp24(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp25(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp26(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 27 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp27(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp28(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp29(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp30(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp31(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DirW {
    bits: u32,
}

impl DirW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DirW { bits: 0 }
    }
    # [ doc = "Bit 0 - Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp16(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp17(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp18(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp19(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp20(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp21(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp22(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp23(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp24(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp25(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp26(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp27(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp28(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp29(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp30(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = input. 1 = output." ]
    pub fn dirp31(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Mask {
    register: ::volatile_register::RW<u32>,
}

impl Mask {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&MaskR, &'w mut MaskW) -> &'w mut MaskW
    {
        let bits = self.register.read();
        let r = MaskR { bits: bits };
        let mut w = MaskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MaskR {
        MaskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MaskW) -> &mut MaskW
    {
        let mut w = MaskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MaskR {
    bits: u32,
}

impl MaskR {
    # [ doc = "Bit 0 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Controls which bits corresponding to GPIOm[n] are active in the P0/1PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp16(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp17(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp18(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp19(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp20(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp21(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp22(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp23(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp24(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp25(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp26(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 27 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp27(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp28(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp29(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp30(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp31(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MaskW {
    bits: u32,
}

impl MaskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MaskW { bits: 0 }
    }
    # [ doc = "Bit 0 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Controls which bits corresponding to GPIOm[n] are active in the P0/1PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp16(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp17(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp18(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp19(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp20(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp21(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp22(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp23(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp24(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp25(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp26(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp27(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp28(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp29(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp30(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Controls which bits corresponding to GPIOm[n] are active in the PIN register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected." ]
    pub fn maskp31(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Pin {
    register: ::volatile_register::RW<u32>,
}

impl Pin {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PinR, &'w mut PinW) -> &'w mut PinW
    {
        let bits = self.register.read();
        let r = PinR { bits: bits };
        let mut w = PinW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PinR {
        PinR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PinW) -> &mut PinW
    {
        let mut w = PinW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PinR {
    bits: u32,
}

impl PinR {
    # [ doc = "Bit 0 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port16(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port17(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port18(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port19(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port20(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port21(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port22(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port23(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port24(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port25(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port26(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 27 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port27(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port28(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port29(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port30(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port31(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PinW {
    bits: u32,
}

impl PinW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PinW { bits: 0 }
    }
    # [ doc = "Bit 0 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port16(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port17(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port18(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port19(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port20(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port21(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port22(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port23(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port24(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port25(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port26(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port27(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port28(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port29(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port30(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit." ]
    pub fn port31(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Mpin {
    register: ::volatile_register::RW<u32>,
}

impl Mpin {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&MpinR, &'w mut MpinW) -> &'w mut MpinW
    {
        let bits = self.register.read();
        let r = MpinR { bits: bits };
        let mut w = MpinW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MpinR {
        MpinR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MpinW) -> &mut MpinW
    {
        let mut w = MpinW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MpinR {
    bits: u32,
}

impl MpinR {
    # [ doc = "Bit 0 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp16(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp17(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp18(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp19(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp20(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp21(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp22(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp23(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp24(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp25(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp26(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 27 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp27(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp28(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp29(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp30(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp31(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MpinW {
    bits: u32,
}

impl MpinW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MpinW { bits: 0 }
    }
    # [ doc = "Bit 0 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp16(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp17(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp18(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp19(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp20(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp21(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp22(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp23(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp24(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp25(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp26(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp27(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp28(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp29(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp30(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31]). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0." ]
    pub fn mportp31(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Set {
    register: ::volatile_register::RW<u32>,
}

impl Set {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&SetR, &'w mut SetW) -> &'w mut SetW
    {
        let bits = self.register.read();
        let r = SetR { bits: bits };
        let mut w = SetW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> SetR {
        SetR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut SetW) -> &mut SetW
    {
        let mut w = SetW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SetR {
    bits: u32,
}

impl SetR {
    # [ doc = "Bit 0 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp16(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp17(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp18(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp19(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp20(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp21(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp22(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp23(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp24(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp25(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp26(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 27 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp27(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp28(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp29(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp30(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp31(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SetW {
    bits: u32,
}

impl SetW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        SetW { bits: 0 }
    }
    # [ doc = "Bit 0 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp16(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp17(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp18(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp19(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp20(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp21(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp22(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp23(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp24(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp25(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp26(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp27(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp28(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp29(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp30(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit." ]
    pub fn setp31(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Clr {
    register: ::volatile_register::WO<u32>,
}

impl Clr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut ClrW) -> &mut ClrW
    {
        let mut w = ClrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClrR {
    bits: u32,
}

impl ClrR {
    # [ doc = "Bit 0 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp00(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp01(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp02(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp03(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp04(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp05(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp06(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp07(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp08(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp09(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp010(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp011(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp012(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp013(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp014(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp015(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp016(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp017(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp018(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp019(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp020(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp021(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp022(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp023(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp024(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp025(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp026(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 27 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp027(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp028(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp029(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp030(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp031(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClrW {
    bits: u32,
}

impl ClrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClrW { bits: 0 }
    }
    # [ doc = "Bit 0 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp00(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp01(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp02(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp03(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp04(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp05(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp06(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp07(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp08(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp09(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp010(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp011(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp012(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp013(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp014(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp015(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp016(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp017(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp018(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp019(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp020(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp021(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp022(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp023(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp024(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp025(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp026(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp027(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp028(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp029(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp030(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Clear output bits: 0 = No operation. 1 = Clear output bit." ]
    pub fn clrp031(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Not {
    register: ::volatile_register::WO<u32>,
}

impl Not {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut NotW) -> &mut NotW
    {
        let mut w = NotW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct NotR {
    bits: u32,
}

impl NotR {
    # [ doc = "Bit 0 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp16(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp17(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp18(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp19(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp20(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp21(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp22(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp23(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp24(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp25(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp26(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 27 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp27(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp28(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp29(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp30(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp31(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct NotW {
    bits: u32,
}

impl NotW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        NotW { bits: 0 }
    }
    # [ doc = "Bit 0 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp16(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp17(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp18(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp19(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp20(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp21(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp22(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp23(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp24(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp25(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp26(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp27(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp28(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp29(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp30(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Toggle output bits: 0 = no operation. 1 = Toggle output bit." ]
    pub fn notp31(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

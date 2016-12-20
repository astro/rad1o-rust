# [ doc = "SSP0/1" ]
# [ repr ( C ) ]
pub struct Ssp0 {
    # [ doc = "0x00 - Control Register 0. Selects the serial clock rate, bus type, and data size." ]
    pub cr0: Cr0,
    # [ doc = "0x04 - Control Register 1. Selects master/slave and other modes." ]
    pub cr1: Cr1,
    # [ doc = "0x08 - Data Register. Writes fill the transmit FIFO, and reads empty the receive FIFO." ]
    pub dr: Dr,
    # [ doc = "0x0c - Status Register" ]
    pub sr: Sr,
    # [ doc = "0x10 - Clock Prescale Register" ]
    pub cpsr: Cpsr,
    # [ doc = "0x14 - Interrupt Mask Set and Clear Register" ]
    pub imsc: Imsc,
    # [ doc = "0x18 - Raw Interrupt Status Register" ]
    pub ris: Ris,
    # [ doc = "0x1c - Masked Interrupt Status Register" ]
    pub mis: Mis,
    # [ doc = "0x20 - SSPICR Interrupt Clear Register" ]
    pub icr: Icr,
    # [ doc = "0x24 - SSP0 DMA control register" ]
    pub dmacr: Dmacr,
}

# [ repr ( C ) ]
pub struct Cr0 {
    register: ::volatile_register::RW<u32>,
}

impl Cr0 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cr0R, &'w mut Cr0W) -> &'w mut Cr0W
    {
        let bits = self.register.read();
        let r = Cr0R { bits: bits };
        let mut w = Cr0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cr0R {
        Cr0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cr0W) -> &mut Cr0W
    {
        let mut w = Cr0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr0R {
    bits: u32,
}

impl Cr0R {
    # [ doc = "Bits 0:3 - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used." ]
    pub fn dss(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:5 - Frame Format." ]
    pub fn frf(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 6 - Clock Out Polarity. This bit is only used in SPI mode." ]
    pub fn cpol(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Clock Out Phase. This bit is only used in SPI mode." ]
    pub fn cpha(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X [SCR+1])." ]
    pub fn scr(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr0W {
    bits: u32,
}

impl Cr0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cr0W { bits: 0 }
    }
    # [ doc = "Bits 0:3 - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used." ]
    pub fn dss(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:5 - Frame Format." ]
    pub fn frf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Clock Out Polarity. This bit is only used in SPI mode." ]
    pub fn cpol(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Clock Out Phase. This bit is only used in SPI mode." ]
    pub fn cpha(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X [SCR+1])." ]
    pub fn scr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cr1 {
    register: ::volatile_register::RW<u32>,
}

impl Cr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cr1R, &'w mut Cr1W) -> &'w mut Cr1W
    {
        let bits = self.register.read();
        let r = Cr1R { bits: bits };
        let mut w = Cr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cr1R {
        Cr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cr1W) -> &mut Cr1W
    {
        let mut w = Cr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr1R {
    bits: u32,
}

impl Cr1R {
    # [ doc = "Bit 0 - Loop Back Mode." ]
    pub fn lbm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - SSP Enable." ]
    pub fn sse(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0." ]
    pub fn ms(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SSP controller from driving the transmit data line (MISO)." ]
    pub fn sod(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&self) -> u32 {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr1W {
    bits: u32,
}

impl Cr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cr1W { bits: 0 }
    }
    # [ doc = "Bit 0 - Loop Back Mode." ]
    pub fn lbm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - SSP Enable." ]
    pub fn sse(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0." ]
    pub fn ms(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SSP controller from driving the transmit data line (MISO)." ]
    pub fn sod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u32 = 268435455;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dr {
    register: ::volatile_register::RW<u32>,
}

impl Dr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DrR, &'w mut DrW) -> &'w mut DrW
    {
        let bits = self.register.read();
        let r = DrR { bits: bits };
        let mut w = DrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DrR {
        DrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DrW) -> &mut DrW
    {
        let mut w = DrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DrR {
    bits: u32,
}

impl DrR {
    # [ doc = "Bits 0:15 - Write: software can write data to be sent in a future frame to this register whenever the TNF bit in the Status register is 1, indicating that the Tx FIFO is not full. If the Tx FIFO was previously empty and the SSP controller is not busy on the bus, transmission of the data will begin immediately. Otherwise the data written to this register will be sent as soon as all previous data has been sent (and received). If the data length is less than 16 bits, software must right-justify the data written to this register. Read: software can read data from this register whenever the RNE bit in the Status register is 1, indicating that the Rx FIFO is not empty. When software reads this register, the SSP controller returns data from the least recent frame in the Rx FIFO. If the data length is less than 16 bits, the data is right-justified in this field with higher order bits filled with 0s." ]
    pub fn data(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DrW {
    bits: u32,
}

impl DrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DrW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Write: software can write data to be sent in a future frame to this register whenever the TNF bit in the Status register is 1, indicating that the Tx FIFO is not full. If the Tx FIFO was previously empty and the SSP controller is not busy on the bus, transmission of the data will begin immediately. Otherwise the data written to this register will be sent as soon as all previous data has been sent (and received). If the data length is less than 16 bits, software must right-justify the data written to this register. Read: software can read data from this register whenever the RNE bit in the Status register is 1, indicating that the Rx FIFO is not empty. When software reads this register, the SSP controller returns data from the least recent frame in the Rx FIFO. If the data length is less than 16 bits, the data is right-justified in this field with higher order bits filled with 0s." ]
    pub fn data(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Sr {
    register: ::volatile_register::RO<u32>,
}

impl Sr {
    pub fn read(&self) -> SrR {
        SrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SrR {
    bits: u32,
}

impl SrR {
    # [ doc = "Bit 0 - Transmit FIFO Empty. This bit is 1 is the Transmit FIFO is empty, 0 if not." ]
    pub fn tfe(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transmit FIFO Not Full. This bit is 0 if the Tx FIFO is full, 1 if not." ]
    pub fn tnf(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Receive FIFO Not Empty. This bit is 0 if the Receive FIFO is empty, 1 if not." ]
    pub fn rne(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Receive FIFO Full. This bit is 1 if the Receive FIFO is full, 0 if not." ]
    pub fn rff(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Busy. This bit is 0 if the SSPn controller is idle, or 1 if it is currently sending/receiving a frame and/or the Tx FIFO is not empty." ]
    pub fn bsy(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 5:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&self) -> u32 {
        const MASK: u32 = 134217727;
        const OFFSET: u8 = 5u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SrW {
    bits: u32,
}

impl SrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        SrW { bits: 3 }
    }
    # [ doc = "Bit 0 - Transmit FIFO Empty. This bit is 1 is the Transmit FIFO is empty, 0 if not." ]
    pub fn tfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transmit FIFO Not Full. This bit is 0 if the Tx FIFO is full, 1 if not." ]
    pub fn tnf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Receive FIFO Not Empty. This bit is 0 if the Receive FIFO is empty, 1 if not." ]
    pub fn rne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Receive FIFO Full. This bit is 1 if the Receive FIFO is full, 0 if not." ]
    pub fn rff(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Busy. This bit is 0 if the SSPn controller is idle, or 1 if it is currently sending/receiving a frame and/or the Tx FIFO is not empty." ]
    pub fn bsy(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 5:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 5u8;
        const MASK: u32 = 134217727;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cpsr {
    register: ::volatile_register::RW<u32>,
}

impl Cpsr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CpsrR, &'w mut CpsrW) -> &'w mut CpsrW
    {
        let bits = self.register.read();
        let r = CpsrR { bits: bits };
        let mut w = CpsrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CpsrR {
        CpsrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CpsrW) -> &mut CpsrW
    {
        let mut w = CpsrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CpsrR {
    bits: u32,
}

impl CpsrR {
    # [ doc = "Bits 0:7 - This even value between 2 and 254, by which PCLK is divided to yield the prescaler output clock. Bit 0 always reads as 0." ]
    pub fn cpsdvsr(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CpsrW {
    bits: u32,
}

impl CpsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CpsrW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - This even value between 2 and 254, by which PCLK is divided to yield the prescaler output clock. Bit 0 always reads as 0." ]
    pub fn cpsdvsr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Imsc {
    register: ::volatile_register::RW<u32>,
}

impl Imsc {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ImscR, &'w mut ImscW) -> &'w mut ImscW
    {
        let bits = self.register.read();
        let r = ImscR { bits: bits };
        let mut w = ImscW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ImscR {
        ImscR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ImscW) -> &mut ImscW
    {
        let mut w = ImscW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ImscR {
    bits: u32,
}

impl ImscR {
    # [ doc = "Bit 0 - Software should set this bit to enable interrupt when a Receive Overrun occurs, that is, when the Rx FIFO is full and another frame is completely received. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs." ]
    pub fn rorim(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Software should set this bit to enable interrupt when a Receive Time-out condition occurs. A Receive Time-out occurs when the Rx FIFO is not empty, and no has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X [SCR+1])." ]
    pub fn rtim(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Software should set this bit to enable interrupt when the Rx FIFO is at least half full." ]
    pub fn rxim(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Software should set this bit to enable interrupt when the Tx FIFO is at least half empty." ]
    pub fn txim(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&self) -> u32 {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ImscW {
    bits: u32,
}

impl ImscW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ImscW { bits: 0 }
    }
    # [ doc = "Bit 0 - Software should set this bit to enable interrupt when a Receive Overrun occurs, that is, when the Rx FIFO is full and another frame is completely received. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs." ]
    pub fn rorim(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Software should set this bit to enable interrupt when a Receive Time-out condition occurs. A Receive Time-out occurs when the Rx FIFO is not empty, and no has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X [SCR+1])." ]
    pub fn rtim(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Software should set this bit to enable interrupt when the Rx FIFO is at least half full." ]
    pub fn rxim(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Software should set this bit to enable interrupt when the Tx FIFO is at least half empty." ]
    pub fn txim(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u32 = 268435455;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ris {
    register: ::volatile_register::RO<u32>,
}

impl Ris {
    pub fn read(&self) -> RisR {
        RisR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RisR {
    bits: u32,
}

impl RisR {
    # [ doc = "Bit 0 - This bit is 1 if another frame was completely received while the RxFIFO was full. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs." ]
    pub fn rorris(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - This bit is 1 if the Rx FIFO is not empty, and has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X [SCR+1])." ]
    pub fn rtris(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - This bit is 1 if the Rx FIFO is at least half full." ]
    pub fn rxris(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - This bit is 1 if the Tx FIFO is at least half empty." ]
    pub fn txris(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&self) -> u32 {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RisW {
    bits: u32,
}

impl RisW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        RisW { bits: 8 }
    }
    # [ doc = "Bit 0 - This bit is 1 if another frame was completely received while the RxFIFO was full. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs." ]
    pub fn rorris(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - This bit is 1 if the Rx FIFO is not empty, and has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X [SCR+1])." ]
    pub fn rtris(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - This bit is 1 if the Rx FIFO is at least half full." ]
    pub fn rxris(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - This bit is 1 if the Tx FIFO is at least half empty." ]
    pub fn txris(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u32 = 268435455;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Mis {
    register: ::volatile_register::RO<u32>,
}

impl Mis {
    pub fn read(&self) -> MisR {
        MisR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MisR {
    bits: u32,
}

impl MisR {
    # [ doc = "Bit 0 - This bit is 1 if another frame was completely received while the RxFIFO was full, and this interrupt is enabled." ]
    pub fn rormis(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - This bit is 1 if the Rx FIFO is not empty, has not been read for a time-out period, and this interrupt is enabled. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X [SCR+1])." ]
    pub fn rtmis(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - This bit is 1 if the Rx FIFO is at least half full, and this interrupt is enabled." ]
    pub fn rxmis(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - This bit is 1 if the Tx FIFO is at least half empty, and this interrupt is enabled." ]
    pub fn txmis(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&self) -> u32 {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MisW {
    bits: u32,
}

impl MisW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MisW { bits: 0 }
    }
    # [ doc = "Bit 0 - This bit is 1 if another frame was completely received while the RxFIFO was full, and this interrupt is enabled." ]
    pub fn rormis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - This bit is 1 if the Rx FIFO is not empty, has not been read for a time-out period, and this interrupt is enabled. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X [SCR+1])." ]
    pub fn rtmis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - This bit is 1 if the Rx FIFO is at least half full, and this interrupt is enabled." ]
    pub fn rxmis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - This bit is 1 if the Tx FIFO is at least half empty, and this interrupt is enabled." ]
    pub fn txmis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u32 = 268435455;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Icr {
    register: ::volatile_register::WO<u32>,
}

impl Icr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut IcrW) -> &mut IcrW
    {
        let mut w = IcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IcrR {
    bits: u32,
}

impl IcrR {
    # [ doc = "Bit 0 - Writing a 1 to this bit clears the frame was received when RxFIFO was full interrupt." ]
    pub fn roric(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Writing a 1 to this bit clears the Rx FIFO was not empty and has not been read for a time-out period interrupt. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR / [SCR+1])." ]
    pub fn rtic(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&self) -> u32 {
        const MASK: u32 = 1073741823;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IcrW {
    bits: u32,
}

impl IcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IcrW { bits: 0 }
    }
    # [ doc = "Bit 0 - Writing a 1 to this bit clears the frame was received when RxFIFO was full interrupt." ]
    pub fn roric(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Writing a 1 to this bit clears the Rx FIFO was not empty and has not been read for a time-out period interrupt. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR / [SCR+1])." ]
    pub fn rtic(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u32 = 1073741823;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dmacr {
    register: ::volatile_register::RW<u32>,
}

impl Dmacr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DmacrR, &'w mut DmacrW) -> &'w mut DmacrW
    {
        let bits = self.register.read();
        let r = DmacrR { bits: bits };
        let mut w = DmacrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DmacrR {
        DmacrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DmacrW) -> &mut DmacrW
    {
        let mut w = DmacrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DmacrR {
    bits: u32,
}

impl DmacrR {
    # [ doc = "Bit 0 - Receive DMA Enable. When this bit is set to one 1, DMA for the receive FIFO is enabled, otherwise receive DMA is disabled." ]
    pub fn rxdmae(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transmit DMA Enable. When this bit is set to one 1, DMA for the transmit FIFO is enabled, otherwise transmit DMA is disabled" ]
    pub fn txdmae(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&self) -> u32 {
        const MASK: u32 = 1073741823;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DmacrW {
    bits: u32,
}

impl DmacrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DmacrW { bits: 0 }
    }
    # [ doc = "Bit 0 - Receive DMA Enable. When this bit is set to one 1, DMA for the receive FIFO is enabled, otherwise receive DMA is disabled." ]
    pub fn rxdmae(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transmit DMA Enable. When this bit is set to one 1, DMA for the transmit FIFO is enabled, otherwise transmit DMA is disabled" ]
    pub fn txdmae(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:31 - Reserved, user software should not write ones to reserved bits. The value read from a reserved bit is not defined." ]
    pub fn reserved(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u32 = 1073741823;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

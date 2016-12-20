# [ doc = "Clock Generation Unit (CGU)" ]
# [ repr ( C ) ]
pub struct Cgu {
    _reserved0: [u8; 20usize],
    # [ doc = "0x14 - Frequency monitor register" ]
    pub freq_mon: FreqMon,
    # [ doc = "0x18 - Crystal oscillator control register" ]
    pub xtal_osc_ctrl: XtalOscCtrl,
    # [ doc = "0x1c - PLL0USB status register" ]
    pub pll0usb_stat: Pll0usbStat,
    # [ doc = "0x20 - PLL0USB control register" ]
    pub pll0usb_ctrl: Pll0usbCtrl,
    # [ doc = "0x24 - PLL0USB M-divider register" ]
    pub pll0usb_mdiv: Pll0usbMdiv,
    # [ doc = "0x28 - PLL0USB N/P-divider register" ]
    pub pll0usb_np_div: Pll0usbNpDiv,
    # [ doc = "0x2c - PLL0AUDIO status register" ]
    pub pll0audio_stat: Pll0audioStat,
    # [ doc = "0x30 - PLL0AUDIO control register" ]
    pub pll0audio_ctrl: Pll0audioCtrl,
    # [ doc = "0x34 - PLL0AUDIO M-divider register" ]
    pub pll0audio_mdiv: Pll0audioMdiv,
    # [ doc = "0x38 - PLL0AUDIO N/P-divider register" ]
    pub pll0audio_np_div: Pll0audioNpDiv,
    # [ doc = "0x3c - PLL0AUDIO fractional divider register" ]
    pub pll0audio_frac: Pll0audioFrac,
    # [ doc = "0x40 - PLL1 status register" ]
    pub pll1_stat: Pll1Stat,
    # [ doc = "0x44 - PLL1 control register" ]
    pub pll1_ctrl: Pll1Ctrl,
    # [ doc = "0x48 - Integer divider A control register" ]
    pub idiva_ctrl: IdivaCtrl,
    # [ doc = "0x4c - Integer divider B control register" ]
    pub idivb_ctrl: IdivbCtrl,
    # [ doc = "0x50 - Integer divider C control register" ]
    pub idivc_ctrl: IdivcCtrl,
    # [ doc = "0x54 - Integer divider D control register" ]
    pub idivd_ctrl: IdivdCtrl,
    # [ doc = "0x58 - Integer divider E control register" ]
    pub idive_ctrl: IdiveCtrl,
    # [ doc = "0x5c - Output stage 0 control register for base clock BASE_SAFE_CLK" ]
    pub base_safe_clk: BaseSafeClk,
    # [ doc = "0x60 - Output stage 1 control register for base clock BASE_USB0_CLK" ]
    pub base_usb0_clk: BaseUsb0Clk,
    # [ doc = "0x64 - Output stage 2 control register for base clock BASE_PERIPH_CLK" ]
    pub base_periph_clk: BasePeriphClk,
    # [ doc = "0x68 - Output stage 3 control register for base clock BASE_USB1_CLK" ]
    pub base_usb1_clk: BaseUsb1Clk,
    # [ doc = "0x6c - Output stage BASE_M4_CLK control register" ]
    pub base_m4_clk: BaseM4Clk,
    # [ doc = "0x70 - Output stage BASE_SPIFI_CLK control register" ]
    pub base_spifi_clk: BaseSpifiClk,
    # [ doc = "0x74 - Output stage BASE_SPI_CLK control register" ]
    pub base_spi_clk: BaseSpiClk,
    # [ doc = "0x78 - Output stage BASE_PHY_RX_CLK control register" ]
    pub base_phy_rx_clk: BasePhyRxClk,
    # [ doc = "0x7c - Output stage BASE_PHY_TX_CLK control register" ]
    pub base_phy_tx_clk: BasePhyTxClk,
    # [ doc = "0x80 - Output stage BASE_APB1_CLK control register" ]
    pub base_apb1_clk: BaseApb1Clk,
    # [ doc = "0x84 - Output stage BASE_APB3_CLK control register" ]
    pub base_apb3_clk: BaseApb3Clk,
    # [ doc = "0x88 - Output stage BASE_LCD_CLK control register" ]
    pub base_lcd_clk: BaseLcdClk,
    _reserved1: [u8; 4usize],
    # [ doc = "0x90 - Output stage BASE_SDIO_CLK control register" ]
    pub base_sdio_clk: BaseSdioClk,
    # [ doc = "0x94 - Output stage BASE_SSP0_CLK control register" ]
    pub base_ssp0_clk: BaseSsp0Clk,
    # [ doc = "0x98 - Output stage BASE_SSP1_CLK control register" ]
    pub base_ssp1_clk: BaseSsp1Clk,
    # [ doc = "0x9c - Output stage BASE_UART0_CLK control register" ]
    pub base_uart0_clk: BaseUart0Clk,
    # [ doc = "0xa0 - Output stage BASE_UART1_CLK control register" ]
    pub base_uart1_clk: BaseUart1Clk,
    # [ doc = "0xa4 - Output stage BASE_UART2_CLK control register" ]
    pub base_uart2_clk: BaseUart2Clk,
    # [ doc = "0xa8 - Output stage BASE_UART3_CLK control register" ]
    pub base_uart3_clk: BaseUart3Clk,
    # [ doc = "0xac - Output stage 20 control register for base clock BASE_OUT_CLK" ]
    pub base_out_clk: BaseOutClk,
    _reserved2: [u8; 16usize],
    # [ doc = "0xc0 - Output stage 25 control register for base clock BASE_AUDIO_CLK" ]
    pub base_audio_clk: BaseAudioClk,
    # [ doc = "0xc4 - Output stage 25 control register for base clock BASE_CGU_OUT0_CLK" ]
    pub base_cgu_out0_clk: BaseCguOut0Clk,
    # [ doc = "0xc8 - Output stage 25 control register for base clock BASE_CGU_OUT1_CLK" ]
    pub base_cgu_out1_clk: BaseCguOut1Clk,
}

# [ repr ( C ) ]
pub struct FreqMon {
    register: ::volatile_register::RW<u32>,
}

impl FreqMon {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FreqMonR, &'w mut FreqMonW) -> &'w mut FreqMonW
    {
        let bits = self.register.read();
        let r = FreqMonR { bits: bits };
        let mut w = FreqMonW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FreqMonR {
        FreqMonR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FreqMonW) -> &mut FreqMonW
    {
        let mut w = FreqMonW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FreqMonR {
    bits: u32,
}

impl FreqMonR {
    # [ doc = "Bits 0:8 - 9-bit reference clock-counter value" ]
    pub fn rcnt(&self) -> u16 {
        const MASK: u32 = 511;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 9:22 - 14-bit selected clock-counter value" ]
    pub fn fcnt(&self) -> u16 {
        const MASK: u32 = 16383;
        const OFFSET: u8 = 9u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 23 - Measure frequency" ]
    pub fn meas(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 24:28 - Clock-source selection for the clock to be measured. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FreqMonW {
    bits: u32,
}

impl FreqMonW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FreqMonW { bits: 0 }
    }
    # [ doc = "Bits 0:8 - 9-bit reference clock-counter value" ]
    pub fn rcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 511;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 9:22 - 14-bit selected clock-counter value" ]
    pub fn fcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 9u8;
        const MASK: u16 = 16383;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 23 - Measure frequency" ]
    pub fn meas(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 24:28 - Clock-source selection for the clock to be measured. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct XtalOscCtrl {
    register: ::volatile_register::RW<u32>,
}

impl XtalOscCtrl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&XtalOscCtrlR, &'w mut XtalOscCtrlW) -> &'w mut XtalOscCtrlW
    {
        let bits = self.register.read();
        let r = XtalOscCtrlR { bits: bits };
        let mut w = XtalOscCtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> XtalOscCtrlR {
        XtalOscCtrlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut XtalOscCtrlW) -> &mut XtalOscCtrlW
    {
        let mut w = XtalOscCtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct XtalOscCtrlR {
    bits: u32,
}

impl XtalOscCtrlR {
    # [ doc = "Bit 0 - Oscillator-pad enable. Do not change the BYPASS and ENABLE bits in one write-action: this will result in unstable device operation!" ]
    pub fn enable(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Configure crystal operation or external-clock input pin XTAL1. Do not change the BYPASS and ENABLE bits in one write-action: this will result in unstable device operation!" ]
    pub fn bypass(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Select frequency range" ]
    pub fn hf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:31 - Reserved" ]
    pub fn reserved(&self) -> u32 {
        const MASK: u32 = 536870911;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct XtalOscCtrlW {
    bits: u32,
}

impl XtalOscCtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        XtalOscCtrlW { bits: 5 }
    }
    # [ doc = "Bit 0 - Oscillator-pad enable. Do not change the BYPASS and ENABLE bits in one write-action: this will result in unstable device operation!" ]
    pub fn enable(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Configure crystal operation or external-clock input pin XTAL1. Do not change the BYPASS and ENABLE bits in one write-action: this will result in unstable device operation!" ]
    pub fn bypass(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Select frequency range" ]
    pub fn hf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 3:31 - Reserved" ]
    pub fn reserved(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u32 = 536870911;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pll0usbStat {
    register: ::volatile_register::RO<u32>,
}

impl Pll0usbStat {
    pub fn read(&self) -> Pll0usbStatR {
        Pll0usbStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0usbStatR {
    bits: u32,
}

impl Pll0usbStatR {
    # [ doc = "Bit 0 - PLL0 lock indicator" ]
    pub fn lock(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - PLL0 free running indicator" ]
    pub fn fr(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:31 - Reserved" ]
    pub fn reserved(&self) -> u32 {
        const MASK: u32 = 1073741823;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0usbStatW {
    bits: u32,
}

impl Pll0usbStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pll0usbStatW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - PLL0 lock indicator" ]
    pub fn lock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - PLL0 free running indicator" ]
    pub fn fr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:31 - Reserved" ]
    pub fn reserved(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u32 = 1073741823;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pll0usbCtrl {
    register: ::volatile_register::RW<u32>,
}

impl Pll0usbCtrl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Pll0usbCtrlR, &'w mut Pll0usbCtrlW) -> &'w mut Pll0usbCtrlW
    {
        let bits = self.register.read();
        let r = Pll0usbCtrlR { bits: bits };
        let mut w = Pll0usbCtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Pll0usbCtrlR {
        Pll0usbCtrlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Pll0usbCtrlW) -> &mut Pll0usbCtrlW
    {
        let mut w = Pll0usbCtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0usbCtrlR {
    bits: u32,
}

impl Pll0usbCtrlR {
    # [ doc = "Bit 0 - PLL0 power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Input clock bypass control" ]
    pub fn bypass(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - PLL0 direct input" ]
    pub fn directi(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - PLL0 direct output" ]
    pub fn directo(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - PLL0 clock enable" ]
    pub fn clken(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Reserved" ]
    pub fn reserved1(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Free running mode" ]
    pub fn frm(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Reserved" ]
    pub fn reserved2(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Reserved. Reads as zero. Do not write one to this register." ]
    pub fn reserved3(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Reserved. Reads as zero. Do not write one to this register." ]
    pub fn reserved4(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Reserved. Reads as zero. Do not write one to this register." ]
    pub fn reserved5(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved6(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved7(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0usbCtrlW {
    bits: u32,
}

impl Pll0usbCtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pll0usbCtrlW { bits: 16777219 }
    }
    # [ doc = "Bit 0 - PLL0 power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Input clock bypass control" ]
    pub fn bypass(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - PLL0 direct input" ]
    pub fn directi(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - PLL0 direct output" ]
    pub fn directo(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - PLL0 clock enable" ]
    pub fn clken(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Reserved" ]
    pub fn reserved8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Free running mode" ]
    pub fn frm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Reserved" ]
    pub fn reserved9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Reserved. Reads as zero. Do not write one to this register." ]
    pub fn reserved10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Reserved. Reads as zero. Do not write one to this register." ]
    pub fn reserved11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Reserved. Reads as zero. Do not write one to this register." ]
    pub fn reserved12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved13(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved14(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pll0usbMdiv {
    register: ::volatile_register::RW<u32>,
}

impl Pll0usbMdiv {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Pll0usbMdivR, &'w mut Pll0usbMdivW) -> &'w mut Pll0usbMdivW
    {
        let bits = self.register.read();
        let r = Pll0usbMdivR { bits: bits };
        let mut w = Pll0usbMdivW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Pll0usbMdivR {
        Pll0usbMdivR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Pll0usbMdivW) -> &mut Pll0usbMdivW
    {
        let mut w = Pll0usbMdivW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0usbMdivR {
    bits: u32,
}

impl Pll0usbMdivR {
    # [ doc = "Bits 0:16 - Decoded M-divider coefficient value. Select values for the M-divider between 1 and 131071." ]
    pub fn mdec(&self) -> u32 {
        const MASK: u32 = 131071;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 17:21 - Bandwidth select P value" ]
    pub fn selp(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:27 - Bandwidth select I value" ]
    pub fn seli(&self) -> u8 {
        const MASK: u32 = 63;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - Bandwidth select R value; SELR = 0." ]
    pub fn selr(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0usbMdivW {
    bits: u32,
}

impl Pll0usbMdivW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pll0usbMdivW { bits: 100162410 }
    }
    # [ doc = "Bits 0:16 - Decoded M-divider coefficient value. Select values for the M-divider between 1 and 131071." ]
    pub fn mdec(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 131071;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 17:21 - Bandwidth select P value" ]
    pub fn selp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 17u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:27 - Bandwidth select I value" ]
    pub fn seli(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 63;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:31 - Bandwidth select R value; SELR = 0." ]
    pub fn selr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pll0usbNpDiv {
    register: ::volatile_register::RW<u32>,
}

impl Pll0usbNpDiv {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Pll0usbNpDivR, &'w mut Pll0usbNpDivW) -> &'w mut Pll0usbNpDivW
    {
        let bits = self.register.read();
        let r = Pll0usbNpDivR { bits: bits };
        let mut w = Pll0usbNpDivW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Pll0usbNpDivR {
        Pll0usbNpDivR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Pll0usbNpDivW) -> &mut Pll0usbNpDivW
    {
        let mut w = Pll0usbNpDivW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0usbNpDivR {
    bits: u32,
}

impl Pll0usbNpDivR {
    # [ doc = "Bits 0:6 - Decoded P-divider coefficient value" ]
    pub fn pdec(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 7:11 - Reserved" ]
    pub fn reserved1(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:21 - Decoded N-divider coefficient value" ]
    pub fn ndec(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 22:31 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0usbNpDivW {
    bits: u32,
}

impl Pll0usbNpDivW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pll0usbNpDivW { bits: 724994 }
    }
    # [ doc = "Bits 0:6 - Decoded P-divider coefficient value" ]
    pub fn pdec(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 7:11 - Reserved" ]
    pub fn reserved1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:21 - Decoded N-divider coefficient value" ]
    pub fn ndec(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:31 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pll0audioStat {
    register: ::volatile_register::RO<u32>,
}

impl Pll0audioStat {
    pub fn read(&self) -> Pll0audioStatR {
        Pll0audioStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0audioStatR {
    bits: u32,
}

impl Pll0audioStatR {
    # [ doc = "Bit 0 - PLL0 lock indicator" ]
    pub fn lock(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - PLL0 free running indicator" ]
    pub fn fr(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:31 - Reserved" ]
    pub fn reserved(&self) -> u32 {
        const MASK: u32 = 1073741823;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0audioStatW {
    bits: u32,
}

impl Pll0audioStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pll0audioStatW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - PLL0 lock indicator" ]
    pub fn lock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - PLL0 free running indicator" ]
    pub fn fr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:31 - Reserved" ]
    pub fn reserved(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u32 = 1073741823;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pll0audioCtrl {
    register: ::volatile_register::RW<u32>,
}

impl Pll0audioCtrl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Pll0audioCtrlR, &'w mut Pll0audioCtrlW) -> &'w mut Pll0audioCtrlW
    {
        let bits = self.register.read();
        let r = Pll0audioCtrlR { bits: bits };
        let mut w = Pll0audioCtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Pll0audioCtrlR {
        Pll0audioCtrlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Pll0audioCtrlW) -> &mut Pll0audioCtrlW
    {
        let mut w = Pll0audioCtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0audioCtrlR {
    bits: u32,
}

impl Pll0audioCtrlR {
    # [ doc = "Bit 0 - PLL0 power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Input clock bypass control" ]
    pub fn bypass(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - PLL0 direct input" ]
    pub fn directi(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - PLL0 direct output" ]
    pub fn directo(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - PLL0 clock enable" ]
    pub fn clken(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Reserved" ]
    pub fn reserved1(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Free running mode" ]
    pub fn frm(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Reserved" ]
    pub fn reserved2(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Reserved. Reads as zero. Do not write one to this register." ]
    pub fn reserved3(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Reserved. Reads as zero. Do not write one to this register." ]
    pub fn reserved4(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Reserved. Reads as zero. Do not write one to this register." ]
    pub fn reserved5(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Fractional PLL word write request. Set this bit to 1 if the fractional divider is enabled in the SEL_EXT bit." ]
    pub fn pllfract_req(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Select fractional divider." ]
    pub fn sel_ext(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Sigma-Delta modulator power-down" ]
    pub fn mod_pd(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 15:23 - Reserved" ]
    pub fn reserved6(&self) -> u16 {
        const MASK: u32 = 511;
        const OFFSET: u8 = 15u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved7(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0audioCtrlW {
    bits: u32,
}

impl Pll0audioCtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pll0audioCtrlW { bits: 16793603 }
    }
    # [ doc = "Bit 0 - PLL0 power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Input clock bypass control" ]
    pub fn bypass(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - PLL0 direct input" ]
    pub fn directi(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - PLL0 direct output" ]
    pub fn directo(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - PLL0 clock enable" ]
    pub fn clken(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Reserved" ]
    pub fn reserved1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Free running mode" ]
    pub fn frm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Reserved" ]
    pub fn reserved2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Reserved. Reads as zero. Do not write one to this register." ]
    pub fn reserved3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Reserved. Reads as zero. Do not write one to this register." ]
    pub fn reserved4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Reserved. Reads as zero. Do not write one to this register." ]
    pub fn reserved5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Fractional PLL word write request. Set this bit to 1 if the fractional divider is enabled in the SEL_EXT bit." ]
    pub fn pllfract_req(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Select fractional divider." ]
    pub fn sel_ext(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Sigma-Delta modulator power-down" ]
    pub fn mod_pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 15:23 - Reserved" ]
    pub fn reserved6(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 15u8;
        const MASK: u16 = 511;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved7(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pll0audioMdiv {
    register: ::volatile_register::RW<u32>,
}

impl Pll0audioMdiv {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Pll0audioMdivR, &'w mut Pll0audioMdivW) -> &'w mut Pll0audioMdivW
    {
        let bits = self.register.read();
        let r = Pll0audioMdivR { bits: bits };
        let mut w = Pll0audioMdivW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Pll0audioMdivR {
        Pll0audioMdivR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Pll0audioMdivW) -> &mut Pll0audioMdivW
    {
        let mut w = Pll0audioMdivW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0audioMdivR {
    bits: u32,
}

impl Pll0audioMdivR {
    # [ doc = "Bits 0:16 - Decoded M-divider coefficient value. Select values for the M-divider between 1 and 131071." ]
    pub fn mdec(&self) -> u32 {
        const MASK: u32 = 131071;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 17:31 - Reserved" ]
    pub fn reserved(&self) -> u16 {
        const MASK: u32 = 32767;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0audioMdivW {
    bits: u32,
}

impl Pll0audioMdivW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pll0audioMdivW { bits: 100162410 }
    }
    # [ doc = "Bits 0:16 - Decoded M-divider coefficient value. Select values for the M-divider between 1 and 131071." ]
    pub fn mdec(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 131071;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 17:31 - Reserved" ]
    pub fn reserved(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 17u8;
        const MASK: u16 = 32767;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pll0audioNpDiv {
    register: ::volatile_register::RW<u32>,
}

impl Pll0audioNpDiv {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & Pll0audioNpDivR , & 'w mut Pll0audioNpDivW ) -> & 'w mut Pll0audioNpDivW , {
        let bits = self.register.read();
        let r = Pll0audioNpDivR { bits: bits };
        let mut w = Pll0audioNpDivW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Pll0audioNpDivR {
        Pll0audioNpDivR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Pll0audioNpDivW) -> &mut Pll0audioNpDivW
    {
        let mut w = Pll0audioNpDivW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0audioNpDivR {
    bits: u32,
}

impl Pll0audioNpDivR {
    # [ doc = "Bits 0:6 - Decoded P-divider coefficient value" ]
    pub fn pdec(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 7:11 - Reserved" ]
    pub fn reserved1(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:21 - Decoded N-divider coefficient value" ]
    pub fn ndec(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 22:31 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0audioNpDivW {
    bits: u32,
}

impl Pll0audioNpDivW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pll0audioNpDivW { bits: 724994 }
    }
    # [ doc = "Bits 0:6 - Decoded P-divider coefficient value" ]
    pub fn pdec(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 7:11 - Reserved" ]
    pub fn reserved1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:21 - Decoded N-divider coefficient value" ]
    pub fn ndec(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:31 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pll0audioFrac {
    register: ::volatile_register::RW<u32>,
}

impl Pll0audioFrac {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Pll0audioFracR, &'w mut Pll0audioFracW) -> &'w mut Pll0audioFracW
    {
        let bits = self.register.read();
        let r = Pll0audioFracR { bits: bits };
        let mut w = Pll0audioFracW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Pll0audioFracR {
        Pll0audioFracR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Pll0audioFracW) -> &mut Pll0audioFracW
    {
        let mut w = Pll0audioFracW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0audioFracR {
    bits: u32,
}

impl Pll0audioFracR {
    # [ doc = "Bits 0:21 - PLL fractional divider control word" ]
    pub fn pllfract_ctrl(&self) -> u32 {
        const MASK: u32 = 4194303;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 22:31 - Reserved" ]
    pub fn reserved(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll0audioFracW {
    bits: u32,
}

impl Pll0audioFracW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pll0audioFracW { bits: 2097152 }
    }
    # [ doc = "Bits 0:21 - PLL fractional divider control word" ]
    pub fn pllfract_ctrl(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4194303;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:31 - Reserved" ]
    pub fn reserved(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pll1Stat {
    register: ::volatile_register::RO<u32>,
}

impl Pll1Stat {
    pub fn read(&self) -> Pll1StatR {
        Pll1StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll1StatR {
    bits: u32,
}

impl Pll1StatR {
    # [ doc = "Bit 0 - PLL1 lock indicator" ]
    pub fn lock(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:31 - Reserved" ]
    pub fn reserved(&self) -> u32 {
        const MASK: u32 = 2147483647;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll1StatW {
    bits: u32,
}

impl Pll1StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pll1StatW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - PLL1 lock indicator" ]
    pub fn lock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:31 - Reserved" ]
    pub fn reserved(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u32 = 2147483647;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pll1Ctrl {
    register: ::volatile_register::RW<u32>,
}

impl Pll1Ctrl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Pll1CtrlR, &'w mut Pll1CtrlW) -> &'w mut Pll1CtrlW
    {
        let bits = self.register.read();
        let r = Pll1CtrlR { bits: bits };
        let mut w = Pll1CtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Pll1CtrlR {
        Pll1CtrlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Pll1CtrlW) -> &mut Pll1CtrlW
    {
        let mut w = Pll1CtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll1CtrlR {
    bits: u32,
}

impl Pll1CtrlR {
    # [ doc = "Bit 0 - PLL1 power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Input clock bypass control" ]
    pub fn bypass(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Reserved. Do not write one to this bit." ]
    pub fn reserved1(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:5 - Reserved. Do not write one to these bits." ]
    pub fn reserved2(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 6 - PLL feedback select." ]
    pub fn fbsel(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - PLL direct CCO output" ]
    pub fn direct(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:9 - Post-divider division ratio P. The value applied is 2xP." ]
    pub fn psel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 10 - Reserved" ]
    pub fn reserved3(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:13 - Pre-divider division ratio N" ]
    pub fn nsel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - Reserved" ]
    pub fn reserved4(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Feedback-divider division ratio (M) 00000000 = 1 00000001 = 2 ... 11111111 = 256" ]
    pub fn msel(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:28 - Clock-source selection." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved5(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pll1CtrlW {
    bits: u32,
}

impl Pll1CtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pll1CtrlW { bits: 16777219 }
    }
    # [ doc = "Bit 0 - PLL1 power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Input clock bypass control" ]
    pub fn bypass(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Reserved. Do not write one to this bit." ]
    pub fn reserved1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 3:5 - Reserved. Do not write one to these bits." ]
    pub fn reserved2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - PLL feedback select." ]
    pub fn fbsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - PLL direct CCO output" ]
    pub fn direct(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:9 - Post-divider division ratio P. The value applied is 2xP." ]
    pub fn psel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Reserved" ]
    pub fn reserved3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:13 - Pre-divider division ratio N" ]
    pub fn nsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - Reserved" ]
    pub fn reserved4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Feedback-divider division ratio (M) 00000000 = 1 00000001 = 2 ... 11111111 = 256" ]
    pub fn msel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock-source selection." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved5(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct IdivaCtrl {
    register: ::volatile_register::RW<u32>,
}

impl IdivaCtrl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IdivaCtrlR, &'w mut IdivaCtrlW) -> &'w mut IdivaCtrlW
    {
        let bits = self.register.read();
        let r = IdivaCtrlR { bits: bits };
        let mut w = IdivaCtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IdivaCtrlR {
        IdivaCtrlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IdivaCtrlW) -> &mut IdivaCtrlW
    {
        let mut w = IdivaCtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdivaCtrlR {
    bits: u32,
}

impl IdivaCtrlR {
    # [ doc = "Bit 0 - Integer divider A power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Reserved" ]
    pub fn reserved1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - Integer divider A divider values (1/(IDIV + 1))" ]
    pub fn idiv(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:10 - Reserved" ]
    pub fn reserved2(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved3(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved4(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdivaCtrlW {
    bits: u32,
}

impl IdivaCtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IdivaCtrlW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Integer divider A power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Reserved" ]
    pub fn reserved1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - Integer divider A divider values (1/(IDIV + 1))" ]
    pub fn idiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:10 - Reserved" ]
    pub fn reserved2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved3(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct IdivbCtrl {
    register: ::volatile_register::RW<u32>,
}

impl IdivbCtrl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IdivbCtrlR, &'w mut IdivbCtrlW) -> &'w mut IdivbCtrlW
    {
        let bits = self.register.read();
        let r = IdivbCtrlR { bits: bits };
        let mut w = IdivbCtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IdivbCtrlR {
        IdivbCtrlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IdivbCtrlW) -> &mut IdivbCtrlW
    {
        let mut w = IdivbCtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdivbCtrlR {
    bits: u32,
}

impl IdivbCtrlR {
    # [ doc = "Bit 0 - Integer divider power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Reserved" ]
    pub fn reserved1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:5 - Integer divider B, C, D divider values (1/(IDIV + 1)) 0000 = 1 (default) 0001 = 2 ... 1111 = 16" ]
    pub fn idiv(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:10 - Reserved" ]
    pub fn reserved2(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved3(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock-source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved4(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdivbCtrlW {
    bits: u32,
}

impl IdivbCtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IdivbCtrlW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Integer divider power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Reserved" ]
    pub fn reserved1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:5 - Integer divider B, C, D divider values (1/(IDIV + 1)) 0000 = 1 (default) 0001 = 2 ... 1111 = 16" ]
    pub fn idiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:10 - Reserved" ]
    pub fn reserved2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved3(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock-source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct IdivcCtrl {
    register: ::volatile_register::RW<u32>,
}

impl IdivcCtrl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IdivcCtrlR, &'w mut IdivcCtrlW) -> &'w mut IdivcCtrlW
    {
        let bits = self.register.read();
        let r = IdivcCtrlR { bits: bits };
        let mut w = IdivcCtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IdivcCtrlR {
        IdivcCtrlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IdivcCtrlW) -> &mut IdivcCtrlW
    {
        let mut w = IdivcCtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdivcCtrlR {
    bits: u32,
}

impl IdivcCtrlR {
    # [ doc = "Bit 0 - Integer divider power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Reserved" ]
    pub fn reserved1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:5 - Integer divider B, C, D divider values (1/(IDIV + 1)) 0000 = 1 (default) 0001 = 2 ... 1111 = 16" ]
    pub fn idiv(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:10 - Reserved" ]
    pub fn reserved2(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved3(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock-source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved4(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdivcCtrlW {
    bits: u32,
}

impl IdivcCtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IdivcCtrlW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Integer divider power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Reserved" ]
    pub fn reserved1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:5 - Integer divider B, C, D divider values (1/(IDIV + 1)) 0000 = 1 (default) 0001 = 2 ... 1111 = 16" ]
    pub fn idiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:10 - Reserved" ]
    pub fn reserved2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved3(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock-source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct IdivdCtrl {
    register: ::volatile_register::RW<u32>,
}

impl IdivdCtrl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IdivdCtrlR, &'w mut IdivdCtrlW) -> &'w mut IdivdCtrlW
    {
        let bits = self.register.read();
        let r = IdivdCtrlR { bits: bits };
        let mut w = IdivdCtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IdivdCtrlR {
        IdivdCtrlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IdivdCtrlW) -> &mut IdivdCtrlW
    {
        let mut w = IdivdCtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdivdCtrlR {
    bits: u32,
}

impl IdivdCtrlR {
    # [ doc = "Bit 0 - Integer divider power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Reserved" ]
    pub fn reserved1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:5 - Integer divider B, C, D divider values (1/(IDIV + 1)) 0000 = 1 (default) 0001 = 2 ... 1111 = 16" ]
    pub fn idiv(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:10 - Reserved" ]
    pub fn reserved2(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved3(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock-source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved4(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdivdCtrlW {
    bits: u32,
}

impl IdivdCtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IdivdCtrlW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Integer divider power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Reserved" ]
    pub fn reserved5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:5 - Integer divider B, C, D divider values (1/(IDIV + 1)) 0000 = 1 (default) 0001 = 2 ... 1111 = 16" ]
    pub fn idiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:10 - Reserved" ]
    pub fn reserved6(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved7(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock-source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved8(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct IdiveCtrl {
    register: ::volatile_register::RW<u32>,
}

impl IdiveCtrl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IdiveCtrlR, &'w mut IdiveCtrlW) -> &'w mut IdiveCtrlW
    {
        let bits = self.register.read();
        let r = IdiveCtrlR { bits: bits };
        let mut w = IdiveCtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IdiveCtrlR {
        IdiveCtrlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IdiveCtrlW) -> &mut IdiveCtrlW
    {
        let mut w = IdiveCtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdiveCtrlR {
    bits: u32,
}

impl IdiveCtrlR {
    # [ doc = "Bit 0 - Integer divider power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Reserved" ]
    pub fn reserved1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:9 - Integer divider E divider values (1/(IDIV + 1)) 00000000 = 1 (default) 00000001 = 2 ... 111111111 = 256" ]
    pub fn idiv(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 10 - Reserved" ]
    pub fn reserved2(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved3(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock-source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved4(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdiveCtrlW {
    bits: u32,
}

impl IdiveCtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IdiveCtrlW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Integer divider power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Reserved" ]
    pub fn reserved1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:9 - Integer divider E divider values (1/(IDIV + 1)) 00000000 = 1 (default) 00000001 = 2 ... 111111111 = 256" ]
    pub fn idiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Reserved" ]
    pub fn reserved2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved3(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock-source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseSafeClk {
    register: ::volatile_register::RO<u32>,
}

impl BaseSafeClk {
    pub fn read(&self) -> BaseSafeClkR {
        BaseSafeClkR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseSafeClkR {
    bits: u32,
}

impl BaseSafeClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseSafeClkW {
    bits: u32,
}

impl BaseSafeClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseSafeClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseUsb0Clk {
    register: ::volatile_register::RW<u32>,
}

impl BaseUsb0Clk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseUsb0ClkR, &'w mut BaseUsb0ClkW) -> &'w mut BaseUsb0ClkW
    {
        let bits = self.register.read();
        let r = BaseUsb0ClkR { bits: bits };
        let mut w = BaseUsb0ClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseUsb0ClkR {
        BaseUsb0ClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseUsb0ClkW) -> &mut BaseUsb0ClkW
    {
        let mut w = BaseUsb0ClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseUsb0ClkR {
    bits: u32,
}

impl BaseUsb0ClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock-source selection." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseUsb0ClkW {
    bits: u32,
}

impl BaseUsb0ClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseUsb0ClkW { bits: 117440512 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock-source selection." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BasePeriphClk {
    register: ::volatile_register::RW<u32>,
}

impl BasePeriphClk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BasePeriphClkR, &'w mut BasePeriphClkW) -> &'w mut BasePeriphClkW
    {
        let bits = self.register.read();
        let r = BasePeriphClkR { bits: bits };
        let mut w = BasePeriphClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BasePeriphClkR {
        BasePeriphClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BasePeriphClkW) -> &mut BasePeriphClkW
    {
        let mut w = BasePeriphClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BasePeriphClkR {
    bits: u32,
}

impl BasePeriphClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BasePeriphClkW {
    bits: u32,
}

impl BasePeriphClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BasePeriphClkW { bits: 117440512 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseUsb1Clk {
    register: ::volatile_register::RW<u32>,
}

impl BaseUsb1Clk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseUsb1ClkR, &'w mut BaseUsb1ClkW) -> &'w mut BaseUsb1ClkW
    {
        let bits = self.register.read();
        let r = BaseUsb1ClkR { bits: bits };
        let mut w = BaseUsb1ClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseUsb1ClkR {
        BaseUsb1ClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseUsb1ClkW) -> &mut BaseUsb1ClkW
    {
        let mut w = BaseUsb1ClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseUsb1ClkR {
    bits: u32,
}

impl BaseUsb1ClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseUsb1ClkW {
    bits: u32,
}

impl BaseUsb1ClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseUsb1ClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseM4Clk {
    register: ::volatile_register::RW<u32>,
}

impl BaseM4Clk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseM4ClkR, &'w mut BaseM4ClkW) -> &'w mut BaseM4ClkW
    {
        let bits = self.register.read();
        let r = BaseM4ClkR { bits: bits };
        let mut w = BaseM4ClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseM4ClkR {
        BaseM4ClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseM4ClkW) -> &mut BaseM4ClkW
    {
        let mut w = BaseM4ClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseM4ClkR {
    bits: u32,
}

impl BaseM4ClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseM4ClkW {
    bits: u32,
}

impl BaseM4ClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseM4ClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseSpifiClk {
    register: ::volatile_register::RW<u32>,
}

impl BaseSpifiClk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseSpifiClkR, &'w mut BaseSpifiClkW) -> &'w mut BaseSpifiClkW
    {
        let bits = self.register.read();
        let r = BaseSpifiClkR { bits: bits };
        let mut w = BaseSpifiClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseSpifiClkR {
        BaseSpifiClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseSpifiClkW) -> &mut BaseSpifiClkW
    {
        let mut w = BaseSpifiClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseSpifiClkR {
    bits: u32,
}

impl BaseSpifiClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseSpifiClkW {
    bits: u32,
}

impl BaseSpifiClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseSpifiClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseSpiClk {
    register: ::volatile_register::RW<u32>,
}

impl BaseSpiClk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseSpiClkR, &'w mut BaseSpiClkW) -> &'w mut BaseSpiClkW
    {
        let bits = self.register.read();
        let r = BaseSpiClkR { bits: bits };
        let mut w = BaseSpiClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseSpiClkR {
        BaseSpiClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseSpiClkW) -> &mut BaseSpiClkW
    {
        let mut w = BaseSpiClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseSpiClkR {
    bits: u32,
}

impl BaseSpiClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseSpiClkW {
    bits: u32,
}

impl BaseSpiClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseSpiClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BasePhyRxClk {
    register: ::volatile_register::RW<u32>,
}

impl BasePhyRxClk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BasePhyRxClkR, &'w mut BasePhyRxClkW) -> &'w mut BasePhyRxClkW
    {
        let bits = self.register.read();
        let r = BasePhyRxClkR { bits: bits };
        let mut w = BasePhyRxClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BasePhyRxClkR {
        BasePhyRxClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BasePhyRxClkW) -> &mut BasePhyRxClkW
    {
        let mut w = BasePhyRxClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BasePhyRxClkR {
    bits: u32,
}

impl BasePhyRxClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BasePhyRxClkW {
    bits: u32,
}

impl BasePhyRxClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BasePhyRxClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BasePhyTxClk {
    register: ::volatile_register::RW<u32>,
}

impl BasePhyTxClk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BasePhyTxClkR, &'w mut BasePhyTxClkW) -> &'w mut BasePhyTxClkW
    {
        let bits = self.register.read();
        let r = BasePhyTxClkR { bits: bits };
        let mut w = BasePhyTxClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BasePhyTxClkR {
        BasePhyTxClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BasePhyTxClkW) -> &mut BasePhyTxClkW
    {
        let mut w = BasePhyTxClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BasePhyTxClkR {
    bits: u32,
}

impl BasePhyTxClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BasePhyTxClkW {
    bits: u32,
}

impl BasePhyTxClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BasePhyTxClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseApb1Clk {
    register: ::volatile_register::RW<u32>,
}

impl BaseApb1Clk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseApb1ClkR, &'w mut BaseApb1ClkW) -> &'w mut BaseApb1ClkW
    {
        let bits = self.register.read();
        let r = BaseApb1ClkR { bits: bits };
        let mut w = BaseApb1ClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseApb1ClkR {
        BaseApb1ClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseApb1ClkW) -> &mut BaseApb1ClkW
    {
        let mut w = BaseApb1ClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseApb1ClkR {
    bits: u32,
}

impl BaseApb1ClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseApb1ClkW {
    bits: u32,
}

impl BaseApb1ClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseApb1ClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseApb3Clk {
    register: ::volatile_register::RW<u32>,
}

impl BaseApb3Clk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseApb3ClkR, &'w mut BaseApb3ClkW) -> &'w mut BaseApb3ClkW
    {
        let bits = self.register.read();
        let r = BaseApb3ClkR { bits: bits };
        let mut w = BaseApb3ClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseApb3ClkR {
        BaseApb3ClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseApb3ClkW) -> &mut BaseApb3ClkW
    {
        let mut w = BaseApb3ClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseApb3ClkR {
    bits: u32,
}

impl BaseApb3ClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseApb3ClkW {
    bits: u32,
}

impl BaseApb3ClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseApb3ClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseLcdClk {
    register: ::volatile_register::RW<u32>,
}

impl BaseLcdClk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseLcdClkR, &'w mut BaseLcdClkW) -> &'w mut BaseLcdClkW
    {
        let bits = self.register.read();
        let r = BaseLcdClkR { bits: bits };
        let mut w = BaseLcdClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseLcdClkR {
        BaseLcdClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseLcdClkW) -> &mut BaseLcdClkW
    {
        let mut w = BaseLcdClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseLcdClkR {
    bits: u32,
}

impl BaseLcdClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseLcdClkW {
    bits: u32,
}

impl BaseLcdClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseLcdClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseSdioClk {
    register: ::volatile_register::RW<u32>,
}

impl BaseSdioClk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseSdioClkR, &'w mut BaseSdioClkW) -> &'w mut BaseSdioClkW
    {
        let bits = self.register.read();
        let r = BaseSdioClkR { bits: bits };
        let mut w = BaseSdioClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseSdioClkR {
        BaseSdioClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseSdioClkW) -> &mut BaseSdioClkW
    {
        let mut w = BaseSdioClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseSdioClkR {
    bits: u32,
}

impl BaseSdioClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseSdioClkW {
    bits: u32,
}

impl BaseSdioClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseSdioClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseSsp0Clk {
    register: ::volatile_register::RW<u32>,
}

impl BaseSsp0Clk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseSsp0ClkR, &'w mut BaseSsp0ClkW) -> &'w mut BaseSsp0ClkW
    {
        let bits = self.register.read();
        let r = BaseSsp0ClkR { bits: bits };
        let mut w = BaseSsp0ClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseSsp0ClkR {
        BaseSsp0ClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseSsp0ClkW) -> &mut BaseSsp0ClkW
    {
        let mut w = BaseSsp0ClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseSsp0ClkR {
    bits: u32,
}

impl BaseSsp0ClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseSsp0ClkW {
    bits: u32,
}

impl BaseSsp0ClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseSsp0ClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseSsp1Clk {
    register: ::volatile_register::RW<u32>,
}

impl BaseSsp1Clk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseSsp1ClkR, &'w mut BaseSsp1ClkW) -> &'w mut BaseSsp1ClkW
    {
        let bits = self.register.read();
        let r = BaseSsp1ClkR { bits: bits };
        let mut w = BaseSsp1ClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseSsp1ClkR {
        BaseSsp1ClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseSsp1ClkW) -> &mut BaseSsp1ClkW
    {
        let mut w = BaseSsp1ClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseSsp1ClkR {
    bits: u32,
}

impl BaseSsp1ClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseSsp1ClkW {
    bits: u32,
}

impl BaseSsp1ClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseSsp1ClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseUart0Clk {
    register: ::volatile_register::RW<u32>,
}

impl BaseUart0Clk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseUart0ClkR, &'w mut BaseUart0ClkW) -> &'w mut BaseUart0ClkW
    {
        let bits = self.register.read();
        let r = BaseUart0ClkR { bits: bits };
        let mut w = BaseUart0ClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseUart0ClkR {
        BaseUart0ClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseUart0ClkW) -> &mut BaseUart0ClkW
    {
        let mut w = BaseUart0ClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseUart0ClkR {
    bits: u32,
}

impl BaseUart0ClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseUart0ClkW {
    bits: u32,
}

impl BaseUart0ClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseUart0ClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseUart1Clk {
    register: ::volatile_register::RW<u32>,
}

impl BaseUart1Clk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseUart1ClkR, &'w mut BaseUart1ClkW) -> &'w mut BaseUart1ClkW
    {
        let bits = self.register.read();
        let r = BaseUart1ClkR { bits: bits };
        let mut w = BaseUart1ClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseUart1ClkR {
        BaseUart1ClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseUart1ClkW) -> &mut BaseUart1ClkW
    {
        let mut w = BaseUart1ClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseUart1ClkR {
    bits: u32,
}

impl BaseUart1ClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseUart1ClkW {
    bits: u32,
}

impl BaseUart1ClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseUart1ClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseUart2Clk {
    register: ::volatile_register::RW<u32>,
}

impl BaseUart2Clk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseUart2ClkR, &'w mut BaseUart2ClkW) -> &'w mut BaseUart2ClkW
    {
        let bits = self.register.read();
        let r = BaseUart2ClkR { bits: bits };
        let mut w = BaseUart2ClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseUart2ClkR {
        BaseUart2ClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseUart2ClkW) -> &mut BaseUart2ClkW
    {
        let mut w = BaseUart2ClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseUart2ClkR {
    bits: u32,
}

impl BaseUart2ClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseUart2ClkW {
    bits: u32,
}

impl BaseUart2ClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseUart2ClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseUart3Clk {
    register: ::volatile_register::RW<u32>,
}

impl BaseUart3Clk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseUart3ClkR, &'w mut BaseUart3ClkW) -> &'w mut BaseUart3ClkW
    {
        let bits = self.register.read();
        let r = BaseUart3ClkR { bits: bits };
        let mut w = BaseUart3ClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseUart3ClkR {
        BaseUart3ClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseUart3ClkW) -> &mut BaseUart3ClkW
    {
        let mut w = BaseUart3ClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseUart3ClkR {
    bits: u32,
}

impl BaseUart3ClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseUart3ClkW {
    bits: u32,
}

impl BaseUart3ClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseUart3ClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock source selection. All other values are reserved." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseOutClk {
    register: ::volatile_register::RW<u32>,
}

impl BaseOutClk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseOutClkR, &'w mut BaseOutClkW) -> &'w mut BaseOutClkW
    {
        let bits = self.register.read();
        let r = BaseOutClkR { bits: bits };
        let mut w = BaseOutClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseOutClkR {
        BaseOutClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseOutClkW) -> &mut BaseOutClkW
    {
        let mut w = BaseOutClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseOutClkR {
    bits: u32,
}

impl BaseOutClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock-source selection." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseOutClkW {
    bits: u32,
}

impl BaseOutClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseOutClkW { bits: 16777216 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock-source selection." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseAudioClk {
    register: ::volatile_register::RW<u32>,
}

impl BaseAudioClk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BaseAudioClkR, &'w mut BaseAudioClkW) -> &'w mut BaseAudioClkW
    {
        let bits = self.register.read();
        let r = BaseAudioClkR { bits: bits };
        let mut w = BaseAudioClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseAudioClkR {
        BaseAudioClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseAudioClkW) -> &mut BaseAudioClkW
    {
        let mut w = BaseAudioClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseAudioClkR {
    bits: u32,
}

impl BaseAudioClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock-source selection." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseAudioClkW {
    bits: u32,
}

impl BaseAudioClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseAudioClkW { bits: 0 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock-source selection." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseCguOut0Clk {
    register: ::volatile_register::RW<u32>,
}

impl BaseCguOut0Clk {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & BaseCguOut0ClkR , & 'w mut BaseCguOut0ClkW ) -> & 'w mut BaseCguOut0ClkW , {
        let bits = self.register.read();
        let r = BaseCguOut0ClkR { bits: bits };
        let mut w = BaseCguOut0ClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseCguOut0ClkR {
        BaseCguOut0ClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseCguOut0ClkW) -> &mut BaseCguOut0ClkW
    {
        let mut w = BaseCguOut0ClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseCguOut0ClkR {
    bits: u32,
}

impl BaseCguOut0ClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock-source selection." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseCguOut0ClkW {
    bits: u32,
}

impl BaseCguOut0ClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseCguOut0ClkW { bits: 0 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock-source selection." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseCguOut1Clk {
    register: ::volatile_register::RW<u32>,
}

impl BaseCguOut1Clk {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & BaseCguOut1ClkR , & 'w mut BaseCguOut1ClkW ) -> & 'w mut BaseCguOut1ClkW , {
        let bits = self.register.read();
        let r = BaseCguOut1ClkR { bits: bits };
        let mut w = BaseCguOut1ClkW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BaseCguOut1ClkR {
        BaseCguOut1ClkR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BaseCguOut1ClkW) -> &mut BaseCguOut1ClkW
    {
        let mut w = BaseCguOut1ClkW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseCguOut1ClkR {
    bits: u32,
}

impl BaseCguOut1ClkR {
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 24:28 - Clock-source selection." ]
    pub fn clk_sel(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseCguOut1ClkW {
    bits: u32,
}

impl BaseCguOut1ClkW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseCguOut1ClkW { bits: 0 }
    }
    # [ doc = "Bit 0 - Output stage power down" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:10 - Reserved" ]
    pub fn reserved1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Block clock automatically during frequency change" ]
    pub fn autoblock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:23 - Reserved" ]
    pub fn reserved2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:28 - Clock-source selection." ]
    pub fn clk_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:31 - Reserved" ]
    pub fn reserved3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ doc = "Clock Control Unit (CCU)" ]
# [ repr ( C ) ]
pub struct Ccu1 {
    # [ doc = "0x00 - CCU1 power mode register" ]
    pub pm: Pm,
    # [ doc = "0x04 - CCU1 base clocks status register" ]
    pub base_stat: BaseStat,
    _reserved0: [u8; 248usize],
    # [ doc = "0x100 - CLK_APB3_BUS clock configuration register" ]
    pub clk_apb3_bus_cfg: ClkApb3BusCfg,
    # [ doc = "0x104 - CLK_APB3_BUS clock status register" ]
    pub clk_apb3_bus_stat: ClkApb3BusStat,
    # [ doc = "0x108 - CLK_APB3_I2C1 clock configuration register" ]
    pub clk_apb3_i2c1_cfg: ClkApb3I2c1Cfg,
    # [ doc = "0x10c - CLK_APB3_I2C1 clock status register" ]
    pub clk_apb3_i2c1_stat: ClkApb3I2c1Stat,
    # [ doc = "0x110 - CLK_APB3_DAC clock configuration register" ]
    pub clk_apb3_dac_cfg: ClkApb3DacCfg,
    # [ doc = "0x114 - CLK_APB3_DAC clock status register" ]
    pub clk_apb3_dac_stat: ClkApb3DacStat,
    # [ doc = "0x118 - CLK_APB3_ADC0 clock configuration register" ]
    pub clk_apb3_adc0_cfg: ClkApb3Adc0Cfg,
    # [ doc = "0x11c - CLK_APB3_ADC0 clock status register" ]
    pub clk_apb3_adc0_stat: ClkApb3Adc0Stat,
    # [ doc = "0x120 - CLK_APB3_ADC1 clock configuration register" ]
    pub clk_apb3_adc1_cfg: ClkApb3Adc1Cfg,
    # [ doc = "0x124 - CLK_APB3_ADC1 clock status register" ]
    pub clk_apb3_adc1_stat: ClkApb3Adc1Stat,
    # [ doc = "0x128 - CLK_APB3_CAN0 clock configuration register" ]
    pub clk_apb3_can0_cfg: ClkApb3Can0Cfg,
    # [ doc = "0x12c - CLK_APB3_CAN0 clock status register" ]
    pub clk_apb3_can0_stat: ClkApb3Can0Stat,
    _reserved1: [u8; 208usize],
    # [ doc = "0x200 - CLK_APB1_BUS clock configuration register" ]
    pub clk_apb1_bus_cfg: ClkApb1BusCfg,
    # [ doc = "0x204 - CLK_APB1_BUS clock status register" ]
    pub clk_apb1_bus_stat: ClkApb1BusStat,
    # [ doc = "0x208 - CLK_APB1_MOTOCONPWM clock configuration register" ]
    pub clk_apb1_motoconpwm_cfg: ClkApb1MotoconpwmCfg,
    # [ doc = "0x20c - CLK_APB1_MOTOCONPWM clock status register" ]
    pub clk_apb1_motoconpwm_stat: ClkApb1MotoconpwmStat,
    # [ doc = "0x210 - CLK_ABP1_I2C0 clock configuration register" ]
    pub clk_apb1_i2c0_cfg: ClkApb1I2c0Cfg,
    # [ doc = "0x214 - CLK_APB1_I2C0 clock status register" ]
    pub clk_apb1_i2c0_stat: ClkApb1I2c0Stat,
    # [ doc = "0x218 - CLK_APB1_I2S clock configuration register" ]
    pub clk_apb1_i2s_cfg: ClkApb1I2sCfg,
    # [ doc = "0x21c - CLK_APB1_I2S clock status register" ]
    pub clk_apb1_i2s_stat: ClkApb1I2sStat,
    # [ doc = "0x220 - CLK_APB1_CAN1 clock configuration register" ]
    pub clk_apb1_can1_cfg: ClkApb1Can1Cfg,
    # [ doc = "0x224 - CLK_APB1_CAN1 clock status register" ]
    pub clk_apb1_can1_stat: ClkApb1Can1Stat,
    _reserved2: [u8; 216usize],
    # [ doc = "0x300 - CLK_SPIFI clock configuration register" ]
    pub clk_spifi_cfg: ClkSpifiCfg,
    # [ doc = "0x304 - CLK_APB1_SPIFI clock status register" ]
    pub clk_spifi_stat: ClkSpifiStat,
    _reserved3: [u8; 248usize],
    # [ doc = "0x400 - CLK_M4_BUS clock configuration register" ]
    pub clk_m4_bus_cfg: ClkM4BusCfg,
    # [ doc = "0x404 - CLK_M4_BUSclock status register" ]
    pub clk_m4_bus_stat: ClkM4BusStat,
    # [ doc = "0x408 - CLK_M4_SPIFI clock configuration register" ]
    pub clk_m4_spifi_cfg: ClkM4SpifiCfg,
    # [ doc = "0x40c - CLK_M4_SPIFI clock status register" ]
    pub clk_m4_spifi_stat: ClkM4SpifiStat,
    # [ doc = "0x410 - CLK_M4_GPIO clock configuration register" ]
    pub clk_m4_gpio_cfg: ClkM4GpioCfg,
    # [ doc = "0x414 - CLK_M4_GPIO clock status register" ]
    pub clk_m4_gpio_stat: ClkM4GpioStat,
    # [ doc = "0x418 - CLK_M4_LCD clock configuration register" ]
    pub clk_m4_lcd_cfg: ClkM4LcdCfg,
    # [ doc = "0x41c - CLK_M4_LCD clock status register" ]
    pub clk_m4_lcd_stat: ClkM4LcdStat,
    # [ doc = "0x420 - CLK_M4_ETHERNET clock configuration register" ]
    pub clk_m4_ethernet_cfg: ClkM4EthernetCfg,
    # [ doc = "0x424 - CLK_M4_ETHERNET clock status register" ]
    pub clk_m4_ethernet_stat: ClkM4EthernetStat,
    # [ doc = "0x428 - CLK_M4_USB0 clock configuration register" ]
    pub clk_m4_usb0_cfg: ClkM4Usb0Cfg,
    # [ doc = "0x42c - CLK_M4_USB0 clock status register" ]
    pub clk_m4_usb0_stat: ClkM4Usb0Stat,
    # [ doc = "0x430 - CLK_M4_EMC clock configuration register" ]
    pub clk_m4_emc_cfg: ClkM4EmcCfg,
    # [ doc = "0x434 - CLK_M4_EMC clock status register" ]
    pub clk_m4_emc_stat: ClkM4EmcStat,
    # [ doc = "0x438 - CLK_M4_SDIO clock configuration register" ]
    pub clk_m4_sdio_cfg: ClkM4SdioCfg,
    # [ doc = "0x43c - CLK_M4_SDIO clock status register" ]
    pub clk_m4_sdio_stat: ClkM4SdioStat,
    # [ doc = "0x440 - CLK_M4_DMA clock configuration register" ]
    pub clk_m4_dma_cfg: ClkM4DmaCfg,
    # [ doc = "0x444 - CLK_M4_DMA clock status register" ]
    pub clk_m4_dma_stat: ClkM4DmaStat,
    # [ doc = "0x448 - CLK_M4_M4CORE clock configuration register" ]
    pub clk_m4_m4core_cfg: ClkM4M4coreCfg,
    # [ doc = "0x44c - CLK_M4_M3CORE clock status register" ]
    pub clk_m4_m4core_stat: ClkM4M4coreStat,
    _reserved4: [u8; 24usize],
    # [ doc = "0x468 - CLK_M4_SCT clock configuration register" ]
    pub clk_m4_sct_cfg: ClkM4SctCfg,
    # [ doc = "0x46c - CLK_M4_SCT clock status register" ]
    pub clk_m4_sct_stat: ClkM4SctStat,
    # [ doc = "0x470 - CLK_M4_USB1 clock configuration register" ]
    pub clk_m4_usb1_cfg: ClkM4Usb1Cfg,
    # [ doc = "0x474 - CLK_M4_USB1 clock status register" ]
    pub clk_m4_usb1_stat: ClkM4Usb1Stat,
    # [ doc = "0x478 - CLK_M4_EMCDIV clock configuration register" ]
    pub clk_m4_emcdiv_cfg: ClkM4EmcdivCfg,
    # [ doc = "0x47c - CLK_M4_EMCDIV clock status register" ]
    pub clk_m4_emcdiv_stat: ClkM4EmcdivStat,
    # [ doc = "0x480 - CLK_M4_FLASHA clock configuration register" ]
    pub clk_m4_flasha_cfg: ClkM4FlashaCfg,
    # [ doc = "0x484 - CLK_M4_FLASHA clock status register" ]
    pub clk_m4_flasha_stat: ClkM4FlashaStat,
    # [ doc = "0x488 - CLK_M4_FLASHB clock configuration register" ]
    pub clk_m4_flashb_cfg: ClkM4FlashbCfg,
    # [ doc = "0x48c - CLK_M4_FLASHB clock status register" ]
    pub clk_m4_flashb_stat: ClkM4FlashbStat,
    # [ doc = "0x490 - CLK_M0APP_CFG clock configuration register" ]
    pub clk_m4_m0app_cfg: ClkM4M0appCfg,
    # [ doc = "0x494 - CLK_M4_MOAPP clock status register" ]
    pub clk_m4_m0app_stat: ClkM4M0appStat,
    # [ doc = "0x498 - CLK_ADCHS_CFG clock configuration register" ]
    pub clk_m4_adchs_cfg: ClkM4AdchsCfg,
    # [ doc = "0x49c - CLK_M4_ADCHS clock status register" ]
    pub clk_m4_adchs_stat: ClkM4AdchsStat,
    # [ doc = "0x4a0 - CLK_EEPROM_CFG clock configuration register" ]
    pub clk_m4_eeprom_cfg: ClkM4EepromCfg,
    # [ doc = "0x4a4 - CLK_M4_EEPROM clock status register" ]
    pub clk_m4_eeprom_stat: ClkM4EepromStat,
    _reserved5: [u8; 88usize],
    # [ doc = "0x500 - CLK_M4_WWDT clock configuration register" ]
    pub clk_m4_wwdt_cfg: ClkM4WwdtCfg,
    # [ doc = "0x504 - CLK_M4_WWDT clock status register" ]
    pub clk_m4_wwdt_stat: ClkM4WwdtStat,
    # [ doc = "0x508 - CLK_M4_USART0 clock configuration register" ]
    pub clk_m4_usart0_cfg: ClkM4Usart0Cfg,
    # [ doc = "0x50c - CLK_M4_USART0 clock status register" ]
    pub clk_m4_usart0_stat: ClkM4Usart0Stat,
    # [ doc = "0x510 - CLK_M4_UART1 clock configuration register" ]
    pub clk_m4_uart1_cfg: ClkM4Uart1Cfg,
    # [ doc = "0x514 - CLK_M4_UART1 clock status register" ]
    pub clk_m4_uart1_stat: ClkM4Uart1Stat,
    # [ doc = "0x518 - CLK_M4_SSP0 clock configuration register" ]
    pub clk_m4_ssp0_cfg: ClkM4Ssp0Cfg,
    # [ doc = "0x51c - CLK_M4_SSP0 clock status register" ]
    pub clk_m4_ssp0_stat: ClkM4Ssp0Stat,
    # [ doc = "0x520 - CLK_M4_TIMER0 clock configuration register" ]
    pub clk_m4_timer0_cfg: ClkM4Timer0Cfg,
    # [ doc = "0x524 - CLK_M4_TIMER0 clock status register" ]
    pub clk_m4_timer0_stat: ClkM4Timer0Stat,
    # [ doc = "0x528 - CLK_M4_TIMER1clock configuration register" ]
    pub clk_m4_timer1_cfg: ClkM4Timer1Cfg,
    # [ doc = "0x52c - CLK_M4_TIMER1 clock status register" ]
    pub clk_m4_timer1_stat: ClkM4Timer1Stat,
    # [ doc = "0x530 - CLK_M4_SCU clock configuration register" ]
    pub clk_m4_scu_cfg: ClkM4ScuCfg,
    # [ doc = "0x534 - CLK_SCU_XXX clock status register" ]
    pub clk_m4_scu_stat: ClkM4ScuStat,
    # [ doc = "0x538 - CLK_M4_CREGclock configuration register" ]
    pub clk_m4_creg_cfg: ClkM4CregCfg,
    # [ doc = "0x53c - CLK_M4_CREG clock status register" ]
    pub clk_m4_creg_stat: ClkM4CregStat,
    _reserved6: [u8; 192usize],
    # [ doc = "0x600 - CLK_M4_RITIMER clock configuration register" ]
    pub clk_m4_ritimer_cfg: ClkM4RitimerCfg,
    # [ doc = "0x604 - CLK_M4_RITIMER clock status register" ]
    pub clk_m4_ritimer_stat: ClkM4RitimerStat,
    # [ doc = "0x608 - CLK_M4_USART2 clock configuration register" ]
    pub clk_m4_usart2_cfg: ClkM4Usart2Cfg,
    # [ doc = "0x60c - CLK_M4_USART2 clock status register" ]
    pub clk_m4_usart2_stat: ClkM4Usart2Stat,
    # [ doc = "0x610 - CLK_M4_USART3 clock configuration register" ]
    pub clk_m4_usart3_cfg: ClkM4Usart3Cfg,
    # [ doc = "0x614 - CLK_M4_USART3 clock status register" ]
    pub clk_m4_usart3_stat: ClkM4Usart3Stat,
    # [ doc = "0x618 - CLK_M4_TIMER2 clock configuration register" ]
    pub clk_m4_timer2_cfg: ClkM4Timer2Cfg,
    # [ doc = "0x61c - CLK_M4_TIMER2 clock status register" ]
    pub clk_m4_timer2_stat: ClkM4Timer2Stat,
    # [ doc = "0x620 - CLK_M4_TIMER3 clock configuration register" ]
    pub clk_m4_timer3_cfg: ClkM4Timer3Cfg,
    # [ doc = "0x624 - CLK_M4_TIMER3 clock status register" ]
    pub clk_m4_timer3_stat: ClkM4Timer3Stat,
    # [ doc = "0x628 - CLK_M4_SSP1 clock configuration register" ]
    pub clk_m4_ssp1_cfg: ClkM4Ssp1Cfg,
    # [ doc = "0x62c - CLK_M4_SSP1 clock status register" ]
    pub clk_m4_ssp1_stat: ClkM4Ssp1Stat,
    # [ doc = "0x630 - CLK_M4_QEIclock configuration register" ]
    pub clk_m4_qei_cfg: ClkM4QeiCfg,
    # [ doc = "0x634 - CLK_M4_QEI clock status register" ]
    pub clk_m4_qei_stat: ClkM4QeiStat,
    _reserved7: [u8; 200usize],
    # [ doc = "0x700 - CLK_PERIPH_BUS_CFG clock configuration register" ]
    pub clk_periph_bus_cfg: ClkPeriphBusCfg,
    # [ doc = "0x704 - CLK_PERIPH_BUS_STAT clock status register" ]
    pub clk_periph_bus_stat: ClkPeriphBusStat,
    _reserved8: [u8; 8usize],
    # [ doc = "0x710 - CLK_PERIPH_CORE_CFG clock configuration register" ]
    pub clk_periph_core_cfg: ClkPeriphCoreCfg,
    # [ doc = "0x714 - CLK_CORE_BUS_STAT clock status register" ]
    pub clk_periph_core_stat: ClkPeriphCoreStat,
    # [ doc = "0x718 - CLK_PERIPH_SGPIO_CFG clock configuration register" ]
    pub clk_periph_sgpio_cfg: ClkPeriphSgpioCfg,
    # [ doc = "0x71c - CLK_CORE_SGPIO_STAT clock status register" ]
    pub clk_periph_sgpio_stat: ClkPeriphSgpioStat,
    _reserved9: [u8; 224usize],
    # [ doc = "0x800 - CLK_M4_USB0 clock configuration register" ]
    pub clk_usb0_cfg: ClkUsb0Cfg,
    # [ doc = "0x804 - CLK_USB0 clock status register" ]
    pub clk_usb0_stat: ClkUsb0Stat,
    _reserved10: [u8; 248usize],
    # [ doc = "0x900 - CLK_USB1 clock configuration register" ]
    pub clk_usb1_cfg: ClkUsb1Cfg,
    # [ doc = "0x904 - CLK_USB1 clock status register" ]
    pub clk_usb1_stat: ClkUsb1Stat,
    _reserved11: [u8; 248usize],
    # [ doc = "0xa00 - CLK_SPI clock configuration register" ]
    pub clk_spi_cfg: ClkSpiCfg,
    # [ doc = "0xa04 - CLK_SPI clock status register" ]
    pub clk_spi_stat: ClkSpiStat,
    _reserved12: [u8; 248usize],
    # [ doc = "0xb00 - CLK_ADCHS clock configuration register" ]
    pub clk_adchs_cfg: ClkAdchsCfg,
    # [ doc = "0xb04 - CLK_ADCHS clock status register" ]
    pub clk_adchs_stat: ClkAdchsStat,
}

# [ repr ( C ) ]
pub struct Pm {
    register: ::volatile_register::RW<u32>,
}

impl Pm {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PmR, &'w mut PmW) -> &'w mut PmW
    {
        let bits = self.register.read();
        let r = PmR { bits: bits };
        let mut w = PmW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PmR {
        PmR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PmW) -> &mut PmW
    {
        let mut w = PmW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PmR {
    bits: u32,
}

impl PmR {
    # [ doc = "Bit 0 - Initiate power-down mode" ]
    pub fn pd(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:31 - Reserved." ]
    pub fn reserved(&self) -> u32 {
        const MASK: u32 = 2147483647;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PmW {
    bits: u32,
}

impl PmW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PmW { bits: 0 }
    }
    # [ doc = "Bit 0 - Initiate power-down mode" ]
    pub fn pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:31 - Reserved." ]
    pub fn reserved(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u32 = 2147483647;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct BaseStat {
    register: ::volatile_register::RO<u32>,
}

impl BaseStat {
    pub fn read(&self) -> BaseStatR {
        BaseStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseStatR {
    bits: u32,
}

impl BaseStatR {
    # [ doc = "Bit 0 - Base clock indicator for BASE_APB3_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running." ]
    pub fn base_apb3_clk_ind(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Base clock indicator for BASE_APB1_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running." ]
    pub fn base_apb1_clk_ind(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Base clock indicator for BASE_SPIFI_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running." ]
    pub fn base_spifi_clk_ind(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Base clock indicator for BASE_M3_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running." ]
    pub fn base_m3_clk_ind(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:6 - Reserved" ]
    pub fn reserved1(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Base clock indicator for BASE_USB0_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running." ]
    pub fn base_usb0_clk_ind(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Base clock indicator for BASE_USB1_CLK 0 = All branch clocks switched off. 1 = at least one branch clock running." ]
    pub fn base_usb1_clk_ind(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 9:31 - Reserved" ]
    pub fn reserved2(&self) -> u32 {
        const MASK: u32 = 8388607;
        const OFFSET: u8 = 9u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BaseStatW {
    bits: u32,
}

impl BaseStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BaseStatW { bits: 4095 }
    }
    # [ doc = "Bit 0 - Base clock indicator for BASE_APB3_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running." ]
    pub fn base_apb3_clk_ind(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Base clock indicator for BASE_APB1_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running." ]
    pub fn base_apb1_clk_ind(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Base clock indicator for BASE_SPIFI_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running." ]
    pub fn base_spifi_clk_ind(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Base clock indicator for BASE_M3_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running." ]
    pub fn base_m3_clk_ind(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:6 - Reserved" ]
    pub fn reserved1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Base clock indicator for BASE_USB0_CLK 0 = All branch clocks switched off. 1 = At least one branch clock running." ]
    pub fn base_usb0_clk_ind(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Base clock indicator for BASE_USB1_CLK 0 = All branch clocks switched off. 1 = at least one branch clock running." ]
    pub fn base_usb1_clk_ind(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 9:31 - Reserved" ]
    pub fn reserved2(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 9u8;
        const MASK: u32 = 8388607;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct ClkApb3BusCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkApb3BusCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkApb3BusCfgR, &'w mut ClkApb3BusCfgW) -> &'w mut ClkApb3BusCfgW
    {
        let bits = self.register.read();
        let r = ClkApb3BusCfgR { bits: bits };
        let mut w = ClkApb3BusCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkApb3BusCfgR {
        ClkApb3BusCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkApb3BusCfgW) -> &mut ClkApb3BusCfgW
    {
        let mut w = ClkApb3BusCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb3BusCfgR {
    bits: u32,
}

impl ClkApb3BusCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb3BusCfgW {
    bits: u32,
}

impl ClkApb3BusCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb3BusCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb3BusStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkApb3BusStat {
    pub fn read(&self) -> ClkApb3BusStatR {
        ClkApb3BusStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb3BusStatR {
    bits: u32,
}

impl ClkApb3BusStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb3BusStatW {
    bits: u32,
}

impl ClkApb3BusStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb3BusStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb3I2c1Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkApb3I2c1Cfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkApb3I2c1CfgR , & 'w mut ClkApb3I2c1CfgW ) -> & 'w mut ClkApb3I2c1CfgW , {
        let bits = self.register.read();
        let r = ClkApb3I2c1CfgR { bits: bits };
        let mut w = ClkApb3I2c1CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkApb3I2c1CfgR {
        ClkApb3I2c1CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkApb3I2c1CfgW) -> &mut ClkApb3I2c1CfgW
    {
        let mut w = ClkApb3I2c1CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb3I2c1CfgR {
    bits: u32,
}

impl ClkApb3I2c1CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb3I2c1CfgW {
    bits: u32,
}

impl ClkApb3I2c1CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb3I2c1CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb3I2c1Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkApb3I2c1Stat {
    pub fn read(&self) -> ClkApb3I2c1StatR {
        ClkApb3I2c1StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb3I2c1StatR {
    bits: u32,
}

impl ClkApb3I2c1StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb3I2c1StatW {
    bits: u32,
}

impl ClkApb3I2c1StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb3I2c1StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb3DacCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkApb3DacCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkApb3DacCfgR, &'w mut ClkApb3DacCfgW) -> &'w mut ClkApb3DacCfgW
    {
        let bits = self.register.read();
        let r = ClkApb3DacCfgR { bits: bits };
        let mut w = ClkApb3DacCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkApb3DacCfgR {
        ClkApb3DacCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkApb3DacCfgW) -> &mut ClkApb3DacCfgW
    {
        let mut w = ClkApb3DacCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb3DacCfgR {
    bits: u32,
}

impl ClkApb3DacCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb3DacCfgW {
    bits: u32,
}

impl ClkApb3DacCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb3DacCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb3DacStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkApb3DacStat {
    pub fn read(&self) -> ClkApb3DacStatR {
        ClkApb3DacStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb3DacStatR {
    bits: u32,
}

impl ClkApb3DacStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb3DacStatW {
    bits: u32,
}

impl ClkApb3DacStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb3DacStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb3Adc0Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkApb3Adc0Cfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkApb3Adc0CfgR , & 'w mut ClkApb3Adc0CfgW ) -> & 'w mut ClkApb3Adc0CfgW , {
        let bits = self.register.read();
        let r = ClkApb3Adc0CfgR { bits: bits };
        let mut w = ClkApb3Adc0CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkApb3Adc0CfgR {
        ClkApb3Adc0CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkApb3Adc0CfgW) -> &mut ClkApb3Adc0CfgW
    {
        let mut w = ClkApb3Adc0CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb3Adc0CfgR {
    bits: u32,
}

impl ClkApb3Adc0CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb3Adc0CfgW {
    bits: u32,
}

impl ClkApb3Adc0CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb3Adc0CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb3Adc0Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkApb3Adc0Stat {
    pub fn read(&self) -> ClkApb3Adc0StatR {
        ClkApb3Adc0StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb3Adc0StatR {
    bits: u32,
}

impl ClkApb3Adc0StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb3Adc0StatW {
    bits: u32,
}

impl ClkApb3Adc0StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb3Adc0StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb3Adc1Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkApb3Adc1Cfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkApb3Adc1CfgR , & 'w mut ClkApb3Adc1CfgW ) -> & 'w mut ClkApb3Adc1CfgW , {
        let bits = self.register.read();
        let r = ClkApb3Adc1CfgR { bits: bits };
        let mut w = ClkApb3Adc1CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkApb3Adc1CfgR {
        ClkApb3Adc1CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkApb3Adc1CfgW) -> &mut ClkApb3Adc1CfgW
    {
        let mut w = ClkApb3Adc1CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb3Adc1CfgR {
    bits: u32,
}

impl ClkApb3Adc1CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb3Adc1CfgW {
    bits: u32,
}

impl ClkApb3Adc1CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb3Adc1CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb3Adc1Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkApb3Adc1Stat {
    pub fn read(&self) -> ClkApb3Adc1StatR {
        ClkApb3Adc1StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb3Adc1StatR {
    bits: u32,
}

impl ClkApb3Adc1StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb3Adc1StatW {
    bits: u32,
}

impl ClkApb3Adc1StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb3Adc1StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb3Can0Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkApb3Can0Cfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkApb3Can0CfgR , & 'w mut ClkApb3Can0CfgW ) -> & 'w mut ClkApb3Can0CfgW , {
        let bits = self.register.read();
        let r = ClkApb3Can0CfgR { bits: bits };
        let mut w = ClkApb3Can0CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkApb3Can0CfgR {
        ClkApb3Can0CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkApb3Can0CfgW) -> &mut ClkApb3Can0CfgW
    {
        let mut w = ClkApb3Can0CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb3Can0CfgR {
    bits: u32,
}

impl ClkApb3Can0CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb3Can0CfgW {
    bits: u32,
}

impl ClkApb3Can0CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb3Can0CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb3Can0Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkApb3Can0Stat {
    pub fn read(&self) -> ClkApb3Can0StatR {
        ClkApb3Can0StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb3Can0StatR {
    bits: u32,
}

impl ClkApb3Can0StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb3Can0StatW {
    bits: u32,
}

impl ClkApb3Can0StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb3Can0StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb1BusCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkApb1BusCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkApb1BusCfgR, &'w mut ClkApb1BusCfgW) -> &'w mut ClkApb1BusCfgW
    {
        let bits = self.register.read();
        let r = ClkApb1BusCfgR { bits: bits };
        let mut w = ClkApb1BusCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkApb1BusCfgR {
        ClkApb1BusCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkApb1BusCfgW) -> &mut ClkApb1BusCfgW
    {
        let mut w = ClkApb1BusCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb1BusCfgR {
    bits: u32,
}

impl ClkApb1BusCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb1BusCfgW {
    bits: u32,
}

impl ClkApb1BusCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb1BusCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb1BusStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkApb1BusStat {
    pub fn read(&self) -> ClkApb1BusStatR {
        ClkApb1BusStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb1BusStatR {
    bits: u32,
}

impl ClkApb1BusStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb1BusStatW {
    bits: u32,
}

impl ClkApb1BusStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb1BusStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb1MotoconpwmCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkApb1MotoconpwmCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkApb1MotoconpwmCfgR, &'w mut ClkApb1MotoconpwmCfgW)
                                -> &'w mut ClkApb1MotoconpwmCfgW
    {
        let bits = self.register.read();
        let r = ClkApb1MotoconpwmCfgR { bits: bits };
        let mut w = ClkApb1MotoconpwmCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkApb1MotoconpwmCfgR {
        ClkApb1MotoconpwmCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkApb1MotoconpwmCfgW) -> &mut ClkApb1MotoconpwmCfgW
    {
        let mut w = ClkApb1MotoconpwmCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb1MotoconpwmCfgR {
    bits: u32,
}

impl ClkApb1MotoconpwmCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb1MotoconpwmCfgW {
    bits: u32,
}

impl ClkApb1MotoconpwmCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb1MotoconpwmCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb1MotoconpwmStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkApb1MotoconpwmStat {
    pub fn read(&self) -> ClkApb1MotoconpwmStatR {
        ClkApb1MotoconpwmStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb1MotoconpwmStatR {
    bits: u32,
}

impl ClkApb1MotoconpwmStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb1MotoconpwmStatW {
    bits: u32,
}

impl ClkApb1MotoconpwmStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb1MotoconpwmStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb1I2c0Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkApb1I2c0Cfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkApb1I2c0CfgR , & 'w mut ClkApb1I2c0CfgW ) -> & 'w mut ClkApb1I2c0CfgW , {
        let bits = self.register.read();
        let r = ClkApb1I2c0CfgR { bits: bits };
        let mut w = ClkApb1I2c0CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkApb1I2c0CfgR {
        ClkApb1I2c0CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkApb1I2c0CfgW) -> &mut ClkApb1I2c0CfgW
    {
        let mut w = ClkApb1I2c0CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb1I2c0CfgR {
    bits: u32,
}

impl ClkApb1I2c0CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb1I2c0CfgW {
    bits: u32,
}

impl ClkApb1I2c0CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb1I2c0CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb1I2c0Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkApb1I2c0Stat {
    pub fn read(&self) -> ClkApb1I2c0StatR {
        ClkApb1I2c0StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb1I2c0StatR {
    bits: u32,
}

impl ClkApb1I2c0StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb1I2c0StatW {
    bits: u32,
}

impl ClkApb1I2c0StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb1I2c0StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb1I2sCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkApb1I2sCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkApb1I2sCfgR, &'w mut ClkApb1I2sCfgW) -> &'w mut ClkApb1I2sCfgW
    {
        let bits = self.register.read();
        let r = ClkApb1I2sCfgR { bits: bits };
        let mut w = ClkApb1I2sCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkApb1I2sCfgR {
        ClkApb1I2sCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkApb1I2sCfgW) -> &mut ClkApb1I2sCfgW
    {
        let mut w = ClkApb1I2sCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb1I2sCfgR {
    bits: u32,
}

impl ClkApb1I2sCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb1I2sCfgW {
    bits: u32,
}

impl ClkApb1I2sCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb1I2sCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb1I2sStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkApb1I2sStat {
    pub fn read(&self) -> ClkApb1I2sStatR {
        ClkApb1I2sStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb1I2sStatR {
    bits: u32,
}

impl ClkApb1I2sStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb1I2sStatW {
    bits: u32,
}

impl ClkApb1I2sStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb1I2sStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb1Can1Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkApb1Can1Cfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkApb1Can1CfgR , & 'w mut ClkApb1Can1CfgW ) -> & 'w mut ClkApb1Can1CfgW , {
        let bits = self.register.read();
        let r = ClkApb1Can1CfgR { bits: bits };
        let mut w = ClkApb1Can1CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkApb1Can1CfgR {
        ClkApb1Can1CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkApb1Can1CfgW) -> &mut ClkApb1Can1CfgW
    {
        let mut w = ClkApb1Can1CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb1Can1CfgR {
    bits: u32,
}

impl ClkApb1Can1CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb1Can1CfgW {
    bits: u32,
}

impl ClkApb1Can1CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb1Can1CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkApb1Can1Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkApb1Can1Stat {
    pub fn read(&self) -> ClkApb1Can1StatR {
        ClkApb1Can1StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkApb1Can1StatR {
    bits: u32,
}

impl ClkApb1Can1StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkApb1Can1StatW {
    bits: u32,
}

impl ClkApb1Can1StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkApb1Can1StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkSpifiCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkSpifiCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkSpifiCfgR, &'w mut ClkSpifiCfgW) -> &'w mut ClkSpifiCfgW
    {
        let bits = self.register.read();
        let r = ClkSpifiCfgR { bits: bits };
        let mut w = ClkSpifiCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkSpifiCfgR {
        ClkSpifiCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkSpifiCfgW) -> &mut ClkSpifiCfgW
    {
        let mut w = ClkSpifiCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkSpifiCfgR {
    bits: u32,
}

impl ClkSpifiCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkSpifiCfgW {
    bits: u32,
}

impl ClkSpifiCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkSpifiCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkSpifiStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkSpifiStat {
    pub fn read(&self) -> ClkSpifiStatR {
        ClkSpifiStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkSpifiStatR {
    bits: u32,
}

impl ClkSpifiStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkSpifiStatW {
    bits: u32,
}

impl ClkSpifiStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkSpifiStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4BusCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4BusCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4BusCfgR, &'w mut ClkM4BusCfgW) -> &'w mut ClkM4BusCfgW
    {
        let bits = self.register.read();
        let r = ClkM4BusCfgR { bits: bits };
        let mut w = ClkM4BusCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4BusCfgR {
        ClkM4BusCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4BusCfgW) -> &mut ClkM4BusCfgW
    {
        let mut w = ClkM4BusCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4BusCfgR {
    bits: u32,
}

impl ClkM4BusCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4BusCfgW {
    bits: u32,
}

impl ClkM4BusCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4BusCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4BusStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4BusStat {
    pub fn read(&self) -> ClkM4BusStatR {
        ClkM4BusStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4BusStatR {
    bits: u32,
}

impl ClkM4BusStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4BusStatW {
    bits: u32,
}

impl ClkM4BusStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4BusStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4SpifiCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4SpifiCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4SpifiCfgR, &'w mut ClkM4SpifiCfgW) -> &'w mut ClkM4SpifiCfgW
    {
        let bits = self.register.read();
        let r = ClkM4SpifiCfgR { bits: bits };
        let mut w = ClkM4SpifiCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4SpifiCfgR {
        ClkM4SpifiCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4SpifiCfgW) -> &mut ClkM4SpifiCfgW
    {
        let mut w = ClkM4SpifiCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4SpifiCfgR {
    bits: u32,
}

impl ClkM4SpifiCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4SpifiCfgW {
    bits: u32,
}

impl ClkM4SpifiCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4SpifiCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4SpifiStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4SpifiStat {
    pub fn read(&self) -> ClkM4SpifiStatR {
        ClkM4SpifiStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4SpifiStatR {
    bits: u32,
}

impl ClkM4SpifiStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4SpifiStatW {
    bits: u32,
}

impl ClkM4SpifiStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4SpifiStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4GpioCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4GpioCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4GpioCfgR, &'w mut ClkM4GpioCfgW) -> &'w mut ClkM4GpioCfgW
    {
        let bits = self.register.read();
        let r = ClkM4GpioCfgR { bits: bits };
        let mut w = ClkM4GpioCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4GpioCfgR {
        ClkM4GpioCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4GpioCfgW) -> &mut ClkM4GpioCfgW
    {
        let mut w = ClkM4GpioCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4GpioCfgR {
    bits: u32,
}

impl ClkM4GpioCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4GpioCfgW {
    bits: u32,
}

impl ClkM4GpioCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4GpioCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4GpioStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4GpioStat {
    pub fn read(&self) -> ClkM4GpioStatR {
        ClkM4GpioStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4GpioStatR {
    bits: u32,
}

impl ClkM4GpioStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4GpioStatW {
    bits: u32,
}

impl ClkM4GpioStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4GpioStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4LcdCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4LcdCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4LcdCfgR, &'w mut ClkM4LcdCfgW) -> &'w mut ClkM4LcdCfgW
    {
        let bits = self.register.read();
        let r = ClkM4LcdCfgR { bits: bits };
        let mut w = ClkM4LcdCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4LcdCfgR {
        ClkM4LcdCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4LcdCfgW) -> &mut ClkM4LcdCfgW
    {
        let mut w = ClkM4LcdCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4LcdCfgR {
    bits: u32,
}

impl ClkM4LcdCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4LcdCfgW {
    bits: u32,
}

impl ClkM4LcdCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4LcdCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4LcdStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4LcdStat {
    pub fn read(&self) -> ClkM4LcdStatR {
        ClkM4LcdStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4LcdStatR {
    bits: u32,
}

impl ClkM4LcdStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4LcdStatW {
    bits: u32,
}

impl ClkM4LcdStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4LcdStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4EthernetCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4EthernetCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4EthernetCfgR, &'w mut ClkM4EthernetCfgW)
                                -> &'w mut ClkM4EthernetCfgW
    {
        let bits = self.register.read();
        let r = ClkM4EthernetCfgR { bits: bits };
        let mut w = ClkM4EthernetCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4EthernetCfgR {
        ClkM4EthernetCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4EthernetCfgW) -> &mut ClkM4EthernetCfgW
    {
        let mut w = ClkM4EthernetCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4EthernetCfgR {
    bits: u32,
}

impl ClkM4EthernetCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4EthernetCfgW {
    bits: u32,
}

impl ClkM4EthernetCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4EthernetCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4EthernetStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4EthernetStat {
    pub fn read(&self) -> ClkM4EthernetStatR {
        ClkM4EthernetStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4EthernetStatR {
    bits: u32,
}

impl ClkM4EthernetStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4EthernetStatW {
    bits: u32,
}

impl ClkM4EthernetStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4EthernetStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Usb0Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4Usb0Cfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4Usb0CfgR, &'w mut ClkM4Usb0CfgW) -> &'w mut ClkM4Usb0CfgW
    {
        let bits = self.register.read();
        let r = ClkM4Usb0CfgR { bits: bits };
        let mut w = ClkM4Usb0CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4Usb0CfgR {
        ClkM4Usb0CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4Usb0CfgW) -> &mut ClkM4Usb0CfgW
    {
        let mut w = ClkM4Usb0CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Usb0CfgR {
    bits: u32,
}

impl ClkM4Usb0CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Usb0CfgW {
    bits: u32,
}

impl ClkM4Usb0CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Usb0CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Usb0Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4Usb0Stat {
    pub fn read(&self) -> ClkM4Usb0StatR {
        ClkM4Usb0StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Usb0StatR {
    bits: u32,
}

impl ClkM4Usb0StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Usb0StatW {
    bits: u32,
}

impl ClkM4Usb0StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Usb0StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4EmcCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4EmcCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4EmcCfgR, &'w mut ClkM4EmcCfgW) -> &'w mut ClkM4EmcCfgW
    {
        let bits = self.register.read();
        let r = ClkM4EmcCfgR { bits: bits };
        let mut w = ClkM4EmcCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4EmcCfgR {
        ClkM4EmcCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4EmcCfgW) -> &mut ClkM4EmcCfgW
    {
        let mut w = ClkM4EmcCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4EmcCfgR {
    bits: u32,
}

impl ClkM4EmcCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4EmcCfgW {
    bits: u32,
}

impl ClkM4EmcCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4EmcCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4EmcStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4EmcStat {
    pub fn read(&self) -> ClkM4EmcStatR {
        ClkM4EmcStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4EmcStatR {
    bits: u32,
}

impl ClkM4EmcStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4EmcStatW {
    bits: u32,
}

impl ClkM4EmcStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4EmcStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4SdioCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4SdioCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4SdioCfgR, &'w mut ClkM4SdioCfgW) -> &'w mut ClkM4SdioCfgW
    {
        let bits = self.register.read();
        let r = ClkM4SdioCfgR { bits: bits };
        let mut w = ClkM4SdioCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4SdioCfgR {
        ClkM4SdioCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4SdioCfgW) -> &mut ClkM4SdioCfgW
    {
        let mut w = ClkM4SdioCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4SdioCfgR {
    bits: u32,
}

impl ClkM4SdioCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4SdioCfgW {
    bits: u32,
}

impl ClkM4SdioCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4SdioCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4SdioStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4SdioStat {
    pub fn read(&self) -> ClkM4SdioStatR {
        ClkM4SdioStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4SdioStatR {
    bits: u32,
}

impl ClkM4SdioStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4SdioStatW {
    bits: u32,
}

impl ClkM4SdioStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4SdioStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4DmaCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4DmaCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4DmaCfgR, &'w mut ClkM4DmaCfgW) -> &'w mut ClkM4DmaCfgW
    {
        let bits = self.register.read();
        let r = ClkM4DmaCfgR { bits: bits };
        let mut w = ClkM4DmaCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4DmaCfgR {
        ClkM4DmaCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4DmaCfgW) -> &mut ClkM4DmaCfgW
    {
        let mut w = ClkM4DmaCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4DmaCfgR {
    bits: u32,
}

impl ClkM4DmaCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4DmaCfgW {
    bits: u32,
}

impl ClkM4DmaCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4DmaCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4DmaStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4DmaStat {
    pub fn read(&self) -> ClkM4DmaStatR {
        ClkM4DmaStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4DmaStatR {
    bits: u32,
}

impl ClkM4DmaStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4DmaStatW {
    bits: u32,
}

impl ClkM4DmaStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4DmaStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4M4coreCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4M4coreCfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkM4M4coreCfgR , & 'w mut ClkM4M4coreCfgW ) -> & 'w mut ClkM4M4coreCfgW , {
        let bits = self.register.read();
        let r = ClkM4M4coreCfgR { bits: bits };
        let mut w = ClkM4M4coreCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4M4coreCfgR {
        ClkM4M4coreCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4M4coreCfgW) -> &mut ClkM4M4coreCfgW
    {
        let mut w = ClkM4M4coreCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4M4coreCfgR {
    bits: u32,
}

impl ClkM4M4coreCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4M4coreCfgW {
    bits: u32,
}

impl ClkM4M4coreCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4M4coreCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4M4coreStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4M4coreStat {
    pub fn read(&self) -> ClkM4M4coreStatR {
        ClkM4M4coreStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4M4coreStatR {
    bits: u32,
}

impl ClkM4M4coreStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4M4coreStatW {
    bits: u32,
}

impl ClkM4M4coreStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4M4coreStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4SctCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4SctCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4SctCfgR, &'w mut ClkM4SctCfgW) -> &'w mut ClkM4SctCfgW
    {
        let bits = self.register.read();
        let r = ClkM4SctCfgR { bits: bits };
        let mut w = ClkM4SctCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4SctCfgR {
        ClkM4SctCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4SctCfgW) -> &mut ClkM4SctCfgW
    {
        let mut w = ClkM4SctCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4SctCfgR {
    bits: u32,
}

impl ClkM4SctCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4SctCfgW {
    bits: u32,
}

impl ClkM4SctCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4SctCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4SctStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4SctStat {
    pub fn read(&self) -> ClkM4SctStatR {
        ClkM4SctStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4SctStatR {
    bits: u32,
}

impl ClkM4SctStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4SctStatW {
    bits: u32,
}

impl ClkM4SctStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4SctStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Usb1Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4Usb1Cfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4Usb1CfgR, &'w mut ClkM4Usb1CfgW) -> &'w mut ClkM4Usb1CfgW
    {
        let bits = self.register.read();
        let r = ClkM4Usb1CfgR { bits: bits };
        let mut w = ClkM4Usb1CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4Usb1CfgR {
        ClkM4Usb1CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4Usb1CfgW) -> &mut ClkM4Usb1CfgW
    {
        let mut w = ClkM4Usb1CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Usb1CfgR {
    bits: u32,
}

impl ClkM4Usb1CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Usb1CfgW {
    bits: u32,
}

impl ClkM4Usb1CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Usb1CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Usb1Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4Usb1Stat {
    pub fn read(&self) -> ClkM4Usb1StatR {
        ClkM4Usb1StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Usb1StatR {
    bits: u32,
}

impl ClkM4Usb1StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Usb1StatW {
    bits: u32,
}

impl ClkM4Usb1StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Usb1StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4EmcdivCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4EmcdivCfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkM4EmcdivCfgR , & 'w mut ClkM4EmcdivCfgW ) -> & 'w mut ClkM4EmcdivCfgW , {
        let bits = self.register.read();
        let r = ClkM4EmcdivCfgR { bits: bits };
        let mut w = ClkM4EmcdivCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4EmcdivCfgR {
        ClkM4EmcdivCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4EmcdivCfgW) -> &mut ClkM4EmcdivCfgW
    {
        let mut w = ClkM4EmcdivCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4EmcdivCfgR {
    bits: u32,
}

impl ClkM4EmcdivCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:4 - Reserved" ]
    pub fn reserved1(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 5:7 - Clock divider value" ]
    pub fn div(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 5u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:31 - Reserved" ]
    pub fn reserved2(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4EmcdivCfgW {
    bits: u32,
}

impl ClkM4EmcdivCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4EmcdivCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 3:4 - Reserved" ]
    pub fn reserved1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 5:7 - Clock divider value" ]
    pub fn div(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 5u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:31 - Reserved" ]
    pub fn reserved2(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct ClkM4EmcdivStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4EmcdivStat {
    pub fn read(&self) -> ClkM4EmcdivStatR {
        ClkM4EmcdivStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4EmcdivStatR {
    bits: u32,
}

impl ClkM4EmcdivStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4EmcdivStatW {
    bits: u32,
}

impl ClkM4EmcdivStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4EmcdivStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4FlashaCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4FlashaCfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkM4FlashaCfgR , & 'w mut ClkM4FlashaCfgW ) -> & 'w mut ClkM4FlashaCfgW , {
        let bits = self.register.read();
        let r = ClkM4FlashaCfgR { bits: bits };
        let mut w = ClkM4FlashaCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4FlashaCfgR {
        ClkM4FlashaCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4FlashaCfgW) -> &mut ClkM4FlashaCfgW
    {
        let mut w = ClkM4FlashaCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4FlashaCfgR {
    bits: u32,
}

impl ClkM4FlashaCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4FlashaCfgW {
    bits: u32,
}

impl ClkM4FlashaCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4FlashaCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4FlashaStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4FlashaStat {
    pub fn read(&self) -> ClkM4FlashaStatR {
        ClkM4FlashaStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4FlashaStatR {
    bits: u32,
}

impl ClkM4FlashaStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4FlashaStatW {
    bits: u32,
}

impl ClkM4FlashaStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4FlashaStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4FlashbCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4FlashbCfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkM4FlashbCfgR , & 'w mut ClkM4FlashbCfgW ) -> & 'w mut ClkM4FlashbCfgW , {
        let bits = self.register.read();
        let r = ClkM4FlashbCfgR { bits: bits };
        let mut w = ClkM4FlashbCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4FlashbCfgR {
        ClkM4FlashbCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4FlashbCfgW) -> &mut ClkM4FlashbCfgW
    {
        let mut w = ClkM4FlashbCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4FlashbCfgR {
    bits: u32,
}

impl ClkM4FlashbCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4FlashbCfgW {
    bits: u32,
}

impl ClkM4FlashbCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4FlashbCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4FlashbStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4FlashbStat {
    pub fn read(&self) -> ClkM4FlashbStatR {
        ClkM4FlashbStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4FlashbStatR {
    bits: u32,
}

impl ClkM4FlashbStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4FlashbStatW {
    bits: u32,
}

impl ClkM4FlashbStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4FlashbStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4M0appCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4M0appCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4M0appCfgR, &'w mut ClkM4M0appCfgW) -> &'w mut ClkM4M0appCfgW
    {
        let bits = self.register.read();
        let r = ClkM4M0appCfgR { bits: bits };
        let mut w = ClkM4M0appCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4M0appCfgR {
        ClkM4M0appCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4M0appCfgW) -> &mut ClkM4M0appCfgW
    {
        let mut w = ClkM4M0appCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4M0appCfgR {
    bits: u32,
}

impl ClkM4M0appCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4M0appCfgW {
    bits: u32,
}

impl ClkM4M0appCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4M0appCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4M0appStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4M0appStat {
    pub fn read(&self) -> ClkM4M0appStatR {
        ClkM4M0appStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4M0appStatR {
    bits: u32,
}

impl ClkM4M0appStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4M0appStatW {
    bits: u32,
}

impl ClkM4M0appStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4M0appStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4AdchsCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4AdchsCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4AdchsCfgR, &'w mut ClkM4AdchsCfgW) -> &'w mut ClkM4AdchsCfgW
    {
        let bits = self.register.read();
        let r = ClkM4AdchsCfgR { bits: bits };
        let mut w = ClkM4AdchsCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4AdchsCfgR {
        ClkM4AdchsCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4AdchsCfgW) -> &mut ClkM4AdchsCfgW
    {
        let mut w = ClkM4AdchsCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4AdchsCfgR {
    bits: u32,
}

impl ClkM4AdchsCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4AdchsCfgW {
    bits: u32,
}

impl ClkM4AdchsCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4AdchsCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4AdchsStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4AdchsStat {
    pub fn read(&self) -> ClkM4AdchsStatR {
        ClkM4AdchsStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4AdchsStatR {
    bits: u32,
}

impl ClkM4AdchsStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4AdchsStatW {
    bits: u32,
}

impl ClkM4AdchsStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4AdchsStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4EepromCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4EepromCfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkM4EepromCfgR , & 'w mut ClkM4EepromCfgW ) -> & 'w mut ClkM4EepromCfgW , {
        let bits = self.register.read();
        let r = ClkM4EepromCfgR { bits: bits };
        let mut w = ClkM4EepromCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4EepromCfgR {
        ClkM4EepromCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4EepromCfgW) -> &mut ClkM4EepromCfgW
    {
        let mut w = ClkM4EepromCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4EepromCfgR {
    bits: u32,
}

impl ClkM4EepromCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4EepromCfgW {
    bits: u32,
}

impl ClkM4EepromCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4EepromCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4EepromStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4EepromStat {
    pub fn read(&self) -> ClkM4EepromStatR {
        ClkM4EepromStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4EepromStatR {
    bits: u32,
}

impl ClkM4EepromStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4EepromStatW {
    bits: u32,
}

impl ClkM4EepromStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4EepromStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4WwdtCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4WwdtCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4WwdtCfgR, &'w mut ClkM4WwdtCfgW) -> &'w mut ClkM4WwdtCfgW
    {
        let bits = self.register.read();
        let r = ClkM4WwdtCfgR { bits: bits };
        let mut w = ClkM4WwdtCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4WwdtCfgR {
        ClkM4WwdtCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4WwdtCfgW) -> &mut ClkM4WwdtCfgW
    {
        let mut w = ClkM4WwdtCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4WwdtCfgR {
    bits: u32,
}

impl ClkM4WwdtCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4WwdtCfgW {
    bits: u32,
}

impl ClkM4WwdtCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4WwdtCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4WwdtStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4WwdtStat {
    pub fn read(&self) -> ClkM4WwdtStatR {
        ClkM4WwdtStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4WwdtStatR {
    bits: u32,
}

impl ClkM4WwdtStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4WwdtStatW {
    bits: u32,
}

impl ClkM4WwdtStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4WwdtStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Usart0Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4Usart0Cfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkM4Usart0CfgR , & 'w mut ClkM4Usart0CfgW ) -> & 'w mut ClkM4Usart0CfgW , {
        let bits = self.register.read();
        let r = ClkM4Usart0CfgR { bits: bits };
        let mut w = ClkM4Usart0CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4Usart0CfgR {
        ClkM4Usart0CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4Usart0CfgW) -> &mut ClkM4Usart0CfgW
    {
        let mut w = ClkM4Usart0CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Usart0CfgR {
    bits: u32,
}

impl ClkM4Usart0CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Usart0CfgW {
    bits: u32,
}

impl ClkM4Usart0CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Usart0CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Usart0Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4Usart0Stat {
    pub fn read(&self) -> ClkM4Usart0StatR {
        ClkM4Usart0StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Usart0StatR {
    bits: u32,
}

impl ClkM4Usart0StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Usart0StatW {
    bits: u32,
}

impl ClkM4Usart0StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Usart0StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Uart1Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4Uart1Cfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4Uart1CfgR, &'w mut ClkM4Uart1CfgW) -> &'w mut ClkM4Uart1CfgW
    {
        let bits = self.register.read();
        let r = ClkM4Uart1CfgR { bits: bits };
        let mut w = ClkM4Uart1CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4Uart1CfgR {
        ClkM4Uart1CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4Uart1CfgW) -> &mut ClkM4Uart1CfgW
    {
        let mut w = ClkM4Uart1CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Uart1CfgR {
    bits: u32,
}

impl ClkM4Uart1CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Uart1CfgW {
    bits: u32,
}

impl ClkM4Uart1CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Uart1CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Uart1Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4Uart1Stat {
    pub fn read(&self) -> ClkM4Uart1StatR {
        ClkM4Uart1StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Uart1StatR {
    bits: u32,
}

impl ClkM4Uart1StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Uart1StatW {
    bits: u32,
}

impl ClkM4Uart1StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Uart1StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Ssp0Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4Ssp0Cfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4Ssp0CfgR, &'w mut ClkM4Ssp0CfgW) -> &'w mut ClkM4Ssp0CfgW
    {
        let bits = self.register.read();
        let r = ClkM4Ssp0CfgR { bits: bits };
        let mut w = ClkM4Ssp0CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4Ssp0CfgR {
        ClkM4Ssp0CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4Ssp0CfgW) -> &mut ClkM4Ssp0CfgW
    {
        let mut w = ClkM4Ssp0CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Ssp0CfgR {
    bits: u32,
}

impl ClkM4Ssp0CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Ssp0CfgW {
    bits: u32,
}

impl ClkM4Ssp0CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Ssp0CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Ssp0Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4Ssp0Stat {
    pub fn read(&self) -> ClkM4Ssp0StatR {
        ClkM4Ssp0StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Ssp0StatR {
    bits: u32,
}

impl ClkM4Ssp0StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Ssp0StatW {
    bits: u32,
}

impl ClkM4Ssp0StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Ssp0StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Timer0Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4Timer0Cfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkM4Timer0CfgR , & 'w mut ClkM4Timer0CfgW ) -> & 'w mut ClkM4Timer0CfgW , {
        let bits = self.register.read();
        let r = ClkM4Timer0CfgR { bits: bits };
        let mut w = ClkM4Timer0CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4Timer0CfgR {
        ClkM4Timer0CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4Timer0CfgW) -> &mut ClkM4Timer0CfgW
    {
        let mut w = ClkM4Timer0CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Timer0CfgR {
    bits: u32,
}

impl ClkM4Timer0CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Timer0CfgW {
    bits: u32,
}

impl ClkM4Timer0CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Timer0CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Timer0Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4Timer0Stat {
    pub fn read(&self) -> ClkM4Timer0StatR {
        ClkM4Timer0StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Timer0StatR {
    bits: u32,
}

impl ClkM4Timer0StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Timer0StatW {
    bits: u32,
}

impl ClkM4Timer0StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Timer0StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Timer1Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4Timer1Cfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkM4Timer1CfgR , & 'w mut ClkM4Timer1CfgW ) -> & 'w mut ClkM4Timer1CfgW , {
        let bits = self.register.read();
        let r = ClkM4Timer1CfgR { bits: bits };
        let mut w = ClkM4Timer1CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4Timer1CfgR {
        ClkM4Timer1CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4Timer1CfgW) -> &mut ClkM4Timer1CfgW
    {
        let mut w = ClkM4Timer1CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Timer1CfgR {
    bits: u32,
}

impl ClkM4Timer1CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Timer1CfgW {
    bits: u32,
}

impl ClkM4Timer1CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Timer1CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Timer1Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4Timer1Stat {
    pub fn read(&self) -> ClkM4Timer1StatR {
        ClkM4Timer1StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Timer1StatR {
    bits: u32,
}

impl ClkM4Timer1StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Timer1StatW {
    bits: u32,
}

impl ClkM4Timer1StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Timer1StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4ScuCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4ScuCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4ScuCfgR, &'w mut ClkM4ScuCfgW) -> &'w mut ClkM4ScuCfgW
    {
        let bits = self.register.read();
        let r = ClkM4ScuCfgR { bits: bits };
        let mut w = ClkM4ScuCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4ScuCfgR {
        ClkM4ScuCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4ScuCfgW) -> &mut ClkM4ScuCfgW
    {
        let mut w = ClkM4ScuCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4ScuCfgR {
    bits: u32,
}

impl ClkM4ScuCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4ScuCfgW {
    bits: u32,
}

impl ClkM4ScuCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4ScuCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4ScuStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4ScuStat {
    pub fn read(&self) -> ClkM4ScuStatR {
        ClkM4ScuStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4ScuStatR {
    bits: u32,
}

impl ClkM4ScuStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4ScuStatW {
    bits: u32,
}

impl ClkM4ScuStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4ScuStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4CregCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4CregCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4CregCfgR, &'w mut ClkM4CregCfgW) -> &'w mut ClkM4CregCfgW
    {
        let bits = self.register.read();
        let r = ClkM4CregCfgR { bits: bits };
        let mut w = ClkM4CregCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4CregCfgR {
        ClkM4CregCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4CregCfgW) -> &mut ClkM4CregCfgW
    {
        let mut w = ClkM4CregCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4CregCfgR {
    bits: u32,
}

impl ClkM4CregCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4CregCfgW {
    bits: u32,
}

impl ClkM4CregCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4CregCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4CregStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4CregStat {
    pub fn read(&self) -> ClkM4CregStatR {
        ClkM4CregStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4CregStatR {
    bits: u32,
}

impl ClkM4CregStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4CregStatW {
    bits: u32,
}

impl ClkM4CregStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4CregStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4RitimerCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4RitimerCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4RitimerCfgR, &'w mut ClkM4RitimerCfgW)
                                -> &'w mut ClkM4RitimerCfgW
    {
        let bits = self.register.read();
        let r = ClkM4RitimerCfgR { bits: bits };
        let mut w = ClkM4RitimerCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4RitimerCfgR {
        ClkM4RitimerCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4RitimerCfgW) -> &mut ClkM4RitimerCfgW
    {
        let mut w = ClkM4RitimerCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4RitimerCfgR {
    bits: u32,
}

impl ClkM4RitimerCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4RitimerCfgW {
    bits: u32,
}

impl ClkM4RitimerCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4RitimerCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4RitimerStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4RitimerStat {
    pub fn read(&self) -> ClkM4RitimerStatR {
        ClkM4RitimerStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4RitimerStatR {
    bits: u32,
}

impl ClkM4RitimerStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4RitimerStatW {
    bits: u32,
}

impl ClkM4RitimerStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4RitimerStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Usart2Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4Usart2Cfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkM4Usart2CfgR , & 'w mut ClkM4Usart2CfgW ) -> & 'w mut ClkM4Usart2CfgW , {
        let bits = self.register.read();
        let r = ClkM4Usart2CfgR { bits: bits };
        let mut w = ClkM4Usart2CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4Usart2CfgR {
        ClkM4Usart2CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4Usart2CfgW) -> &mut ClkM4Usart2CfgW
    {
        let mut w = ClkM4Usart2CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Usart2CfgR {
    bits: u32,
}

impl ClkM4Usart2CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Usart2CfgW {
    bits: u32,
}

impl ClkM4Usart2CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Usart2CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Usart2Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4Usart2Stat {
    pub fn read(&self) -> ClkM4Usart2StatR {
        ClkM4Usart2StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Usart2StatR {
    bits: u32,
}

impl ClkM4Usart2StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Usart2StatW {
    bits: u32,
}

impl ClkM4Usart2StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Usart2StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Usart3Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4Usart3Cfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkM4Usart3CfgR , & 'w mut ClkM4Usart3CfgW ) -> & 'w mut ClkM4Usart3CfgW , {
        let bits = self.register.read();
        let r = ClkM4Usart3CfgR { bits: bits };
        let mut w = ClkM4Usart3CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4Usart3CfgR {
        ClkM4Usart3CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4Usart3CfgW) -> &mut ClkM4Usart3CfgW
    {
        let mut w = ClkM4Usart3CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Usart3CfgR {
    bits: u32,
}

impl ClkM4Usart3CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Usart3CfgW {
    bits: u32,
}

impl ClkM4Usart3CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Usart3CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Usart3Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4Usart3Stat {
    pub fn read(&self) -> ClkM4Usart3StatR {
        ClkM4Usart3StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Usart3StatR {
    bits: u32,
}

impl ClkM4Usart3StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Usart3StatW {
    bits: u32,
}

impl ClkM4Usart3StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Usart3StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Timer2Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4Timer2Cfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkM4Timer2CfgR , & 'w mut ClkM4Timer2CfgW ) -> & 'w mut ClkM4Timer2CfgW , {
        let bits = self.register.read();
        let r = ClkM4Timer2CfgR { bits: bits };
        let mut w = ClkM4Timer2CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4Timer2CfgR {
        ClkM4Timer2CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4Timer2CfgW) -> &mut ClkM4Timer2CfgW
    {
        let mut w = ClkM4Timer2CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Timer2CfgR {
    bits: u32,
}

impl ClkM4Timer2CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Timer2CfgW {
    bits: u32,
}

impl ClkM4Timer2CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Timer2CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Timer2Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4Timer2Stat {
    pub fn read(&self) -> ClkM4Timer2StatR {
        ClkM4Timer2StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Timer2StatR {
    bits: u32,
}

impl ClkM4Timer2StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Timer2StatW {
    bits: u32,
}

impl ClkM4Timer2StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Timer2StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Timer3Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4Timer3Cfg {
    pub fn modify < F > ( & mut self , f : F ) where for < 'w > F : FnOnce ( & ClkM4Timer3CfgR , & 'w mut ClkM4Timer3CfgW ) -> & 'w mut ClkM4Timer3CfgW , {
        let bits = self.register.read();
        let r = ClkM4Timer3CfgR { bits: bits };
        let mut w = ClkM4Timer3CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4Timer3CfgR {
        ClkM4Timer3CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4Timer3CfgW) -> &mut ClkM4Timer3CfgW
    {
        let mut w = ClkM4Timer3CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Timer3CfgR {
    bits: u32,
}

impl ClkM4Timer3CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Timer3CfgW {
    bits: u32,
}

impl ClkM4Timer3CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Timer3CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Timer3Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4Timer3Stat {
    pub fn read(&self) -> ClkM4Timer3StatR {
        ClkM4Timer3StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Timer3StatR {
    bits: u32,
}

impl ClkM4Timer3StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Timer3StatW {
    bits: u32,
}

impl ClkM4Timer3StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Timer3StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Ssp1Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4Ssp1Cfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4Ssp1CfgR, &'w mut ClkM4Ssp1CfgW) -> &'w mut ClkM4Ssp1CfgW
    {
        let bits = self.register.read();
        let r = ClkM4Ssp1CfgR { bits: bits };
        let mut w = ClkM4Ssp1CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4Ssp1CfgR {
        ClkM4Ssp1CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4Ssp1CfgW) -> &mut ClkM4Ssp1CfgW
    {
        let mut w = ClkM4Ssp1CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Ssp1CfgR {
    bits: u32,
}

impl ClkM4Ssp1CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Ssp1CfgW {
    bits: u32,
}

impl ClkM4Ssp1CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Ssp1CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4Ssp1Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4Ssp1Stat {
    pub fn read(&self) -> ClkM4Ssp1StatR {
        ClkM4Ssp1StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4Ssp1StatR {
    bits: u32,
}

impl ClkM4Ssp1StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4Ssp1StatW {
    bits: u32,
}

impl ClkM4Ssp1StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4Ssp1StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4QeiCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkM4QeiCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkM4QeiCfgR, &'w mut ClkM4QeiCfgW) -> &'w mut ClkM4QeiCfgW
    {
        let bits = self.register.read();
        let r = ClkM4QeiCfgR { bits: bits };
        let mut w = ClkM4QeiCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkM4QeiCfgR {
        ClkM4QeiCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkM4QeiCfgW) -> &mut ClkM4QeiCfgW
    {
        let mut w = ClkM4QeiCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4QeiCfgR {
    bits: u32,
}

impl ClkM4QeiCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4QeiCfgW {
    bits: u32,
}

impl ClkM4QeiCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4QeiCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkM4QeiStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkM4QeiStat {
    pub fn read(&self) -> ClkM4QeiStatR {
        ClkM4QeiStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkM4QeiStatR {
    bits: u32,
}

impl ClkM4QeiStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkM4QeiStatW {
    bits: u32,
}

impl ClkM4QeiStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkM4QeiStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkPeriphBusCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkPeriphBusCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkPeriphBusCfgR, &'w mut ClkPeriphBusCfgW)
                                -> &'w mut ClkPeriphBusCfgW
    {
        let bits = self.register.read();
        let r = ClkPeriphBusCfgR { bits: bits };
        let mut w = ClkPeriphBusCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkPeriphBusCfgR {
        ClkPeriphBusCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkPeriphBusCfgW) -> &mut ClkPeriphBusCfgW
    {
        let mut w = ClkPeriphBusCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkPeriphBusCfgR {
    bits: u32,
}

impl ClkPeriphBusCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkPeriphBusCfgW {
    bits: u32,
}

impl ClkPeriphBusCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkPeriphBusCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkPeriphBusStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkPeriphBusStat {
    pub fn read(&self) -> ClkPeriphBusStatR {
        ClkPeriphBusStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkPeriphBusStatR {
    bits: u32,
}

impl ClkPeriphBusStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkPeriphBusStatW {
    bits: u32,
}

impl ClkPeriphBusStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkPeriphBusStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkPeriphCoreCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkPeriphCoreCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkPeriphCoreCfgR, &'w mut ClkPeriphCoreCfgW)
                                -> &'w mut ClkPeriphCoreCfgW
    {
        let bits = self.register.read();
        let r = ClkPeriphCoreCfgR { bits: bits };
        let mut w = ClkPeriphCoreCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkPeriphCoreCfgR {
        ClkPeriphCoreCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkPeriphCoreCfgW) -> &mut ClkPeriphCoreCfgW
    {
        let mut w = ClkPeriphCoreCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkPeriphCoreCfgR {
    bits: u32,
}

impl ClkPeriphCoreCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkPeriphCoreCfgW {
    bits: u32,
}

impl ClkPeriphCoreCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkPeriphCoreCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkPeriphCoreStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkPeriphCoreStat {
    pub fn read(&self) -> ClkPeriphCoreStatR {
        ClkPeriphCoreStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkPeriphCoreStatR {
    bits: u32,
}

impl ClkPeriphCoreStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkPeriphCoreStatW {
    bits: u32,
}

impl ClkPeriphCoreStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkPeriphCoreStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkPeriphSgpioCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkPeriphSgpioCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkPeriphSgpioCfgR, &'w mut ClkPeriphSgpioCfgW)
                                -> &'w mut ClkPeriphSgpioCfgW
    {
        let bits = self.register.read();
        let r = ClkPeriphSgpioCfgR { bits: bits };
        let mut w = ClkPeriphSgpioCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkPeriphSgpioCfgR {
        ClkPeriphSgpioCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkPeriphSgpioCfgW) -> &mut ClkPeriphSgpioCfgW
    {
        let mut w = ClkPeriphSgpioCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkPeriphSgpioCfgR {
    bits: u32,
}

impl ClkPeriphSgpioCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkPeriphSgpioCfgW {
    bits: u32,
}

impl ClkPeriphSgpioCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkPeriphSgpioCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkPeriphSgpioStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkPeriphSgpioStat {
    pub fn read(&self) -> ClkPeriphSgpioStatR {
        ClkPeriphSgpioStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkPeriphSgpioStatR {
    bits: u32,
}

impl ClkPeriphSgpioStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkPeriphSgpioStatW {
    bits: u32,
}

impl ClkPeriphSgpioStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkPeriphSgpioStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkUsb0Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkUsb0Cfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkUsb0CfgR, &'w mut ClkUsb0CfgW) -> &'w mut ClkUsb0CfgW
    {
        let bits = self.register.read();
        let r = ClkUsb0CfgR { bits: bits };
        let mut w = ClkUsb0CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkUsb0CfgR {
        ClkUsb0CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkUsb0CfgW) -> &mut ClkUsb0CfgW
    {
        let mut w = ClkUsb0CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkUsb0CfgR {
    bits: u32,
}

impl ClkUsb0CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkUsb0CfgW {
    bits: u32,
}

impl ClkUsb0CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkUsb0CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkUsb0Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkUsb0Stat {
    pub fn read(&self) -> ClkUsb0StatR {
        ClkUsb0StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkUsb0StatR {
    bits: u32,
}

impl ClkUsb0StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkUsb0StatW {
    bits: u32,
}

impl ClkUsb0StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkUsb0StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkUsb1Cfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkUsb1Cfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkUsb1CfgR, &'w mut ClkUsb1CfgW) -> &'w mut ClkUsb1CfgW
    {
        let bits = self.register.read();
        let r = ClkUsb1CfgR { bits: bits };
        let mut w = ClkUsb1CfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkUsb1CfgR {
        ClkUsb1CfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkUsb1CfgW) -> &mut ClkUsb1CfgW
    {
        let mut w = ClkUsb1CfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkUsb1CfgR {
    bits: u32,
}

impl ClkUsb1CfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkUsb1CfgW {
    bits: u32,
}

impl ClkUsb1CfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkUsb1CfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkUsb1Stat {
    register: ::volatile_register::RO<u32>,
}

impl ClkUsb1Stat {
    pub fn read(&self) -> ClkUsb1StatR {
        ClkUsb1StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkUsb1StatR {
    bits: u32,
}

impl ClkUsb1StatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkUsb1StatW {
    bits: u32,
}

impl ClkUsb1StatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkUsb1StatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkSpiCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkSpiCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkSpiCfgR, &'w mut ClkSpiCfgW) -> &'w mut ClkSpiCfgW
    {
        let bits = self.register.read();
        let r = ClkSpiCfgR { bits: bits };
        let mut w = ClkSpiCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkSpiCfgR {
        ClkSpiCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkSpiCfgW) -> &mut ClkSpiCfgW
    {
        let mut w = ClkSpiCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkSpiCfgR {
    bits: u32,
}

impl ClkSpiCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkSpiCfgW {
    bits: u32,
}

impl ClkSpiCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkSpiCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkSpiStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkSpiStat {
    pub fn read(&self) -> ClkSpiStatR {
        ClkSpiStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkSpiStatR {
    bits: u32,
}

impl ClkSpiStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkSpiStatW {
    bits: u32,
}

impl ClkSpiStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkSpiStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkAdchsCfg {
    register: ::volatile_register::RW<u32>,
}

impl ClkAdchsCfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkAdchsCfgR, &'w mut ClkAdchsCfgW) -> &'w mut ClkAdchsCfgW
    {
        let bits = self.register.read();
        let r = ClkAdchsCfgR { bits: bits };
        let mut w = ClkAdchsCfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkAdchsCfgR {
        ClkAdchsCfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkAdchsCfgW) -> &mut ClkAdchsCfgW
    {
        let mut w = ClkAdchsCfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkAdchsCfgR {
    bits: u32,
}

impl ClkAdchsCfgR {
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkAdchsCfgW {
    bits: u32,
}

impl ClkAdchsCfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkAdchsCfgW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable" ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable" ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable" ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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
pub struct ClkAdchsStat {
    register: ::volatile_register::RO<u32>,
}

impl ClkAdchsStat {
    pub fn read(&self) -> ClkAdchsStatR {
        ClkAdchsStatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkAdchsStatR {
    bits: u32,
}

impl ClkAdchsStatR {
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&self) -> bool {
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
pub struct ClkAdchsStatW {
    bits: u32,
}

impl ClkAdchsStatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkAdchsStatW { bits: 1 }
    }
    # [ doc = "Bit 0 - Run enable status 0 = clock is disabled. 1 = clock is enabled." ]
    pub fn run(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is enabled." ]
    pub fn auto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled." ]
    pub fn wakeup(&mut self, value: bool) -> &mut Self {
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

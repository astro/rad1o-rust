mod gpio_port;

const SCT: usize = 0x40000000;
const GPDMA: usize = 0x40002000;
const SPIFI: usize = 0x40003000;
const SDMMC: usize = 0x40004000;
const EMC: usize = 0x40005000;
const USB0: usize = 0x40006000;
const USB1: usize = 0x40007000;
const LCD: usize = 0x40008000;
const EEPROM: usize = 0x4000e000;
const ETHERNET: usize = 0x40010000;
const ATIMER: usize = 0x40040000;
const REGFILE: usize = 0x40041000;
const PMC: usize = 0x40042000;
const CREG: usize = 0x40043000;
const EVENTROUTER: usize = 0x40044000;
const RTC: usize = 0x40046000;
const CGU: usize = 0x40050000;
const CCU1: usize = 0x40051000;
const CCU2: usize = 0x40052000;
const RGU: usize = 0x40053000;
const WWDT: usize = 0x40080000;
const USART0: usize = 0x40081000;
const USART2: usize = 0x400c1000;
const USART3: usize = 0x400c2000;
const UART1: usize = 0x40082000;
const SSP0: usize = 0x40083000;
const SSP1: usize = 0x400c5000;
const TIMER0: usize = 0x40084000;
const TIMER1: usize = 0x40085000;
const TIMER2: usize = 0x400c3000;
const TIMER3: usize = 0x400c4000;
const SCU: usize = 0x40086000;
const GPIO_PIN_INT: usize = 0x40087000;
const GPIO_GROUP_INT0: usize = 0x40088000;
const GPIO_GROUP_INT1: usize = 0x40089000;
const MCPWM: usize = 0x400a0000;
const I2C0: usize = 0x400a1000;
const I2C1: usize = 0x400e0000;
const I2S0: usize = 0x400a2000;
const I2S1: usize = 0x400a3000;
const C_CAN1: usize = 0x400a4000;
const RITIMER: usize = 0x400c0000;
const QEI: usize = 0x400c6000;
const GIMA: usize = 0x400c7000;
const DAC: usize = 0x400e1000;
const C_CAN0: usize = 0x400e2000;
const ADC0: usize = 0x400e3000;
const ADC1: usize = 0x400e4000;
const ADCHS: usize = 0x400f0000;
const GPIO_PORT: usize = 0x400f4000;
const SPI: usize = 0x40100000;
const SGPIO: usize = 0x40101000;

pub fn gpio_port() -> &'static mut gpio_port::GpioPort {
    unsafe { deref_mut(GPIO_PORT) }
}

unsafe fn deref<T>(address: usize) -> &'static T {
    &*(address as *const T)
}

unsafe fn deref_mut<T>(address: usize) -> &'static mut T {
    &mut *(address as *mut T)
}

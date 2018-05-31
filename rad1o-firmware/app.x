MEMORY
{
        /* rom is really the shadow region that points to SPI flash or elsewhere */
        shadow     (rx)  : ORIGIN = 0x00000000, LENGTH = 128K
        ram_local1 (rwx) : ORIGIN = 0x10000000, LENGTH = 128K
        ram_l0dable(rwx) : ORIGIN = 0x10080000, LENGTH = 16K
        ram_local2 (rwx) : ORIGIN = 0x10084000, LENGTH = 48K
        ram_sleep  (rwx) : ORIGIN = 0x10090000, LENGTH = 8K
}

MEMORY
{
        /* Physical address in Flash used to copy Code from Flash to RAM */
        rom_flash  (rx)  : ORIGIN = 0x80000000, LENGTH =  1M
        ram_m0     (rwx) : ORIGIN = 0x20000000, LENGTH = 28K
        ram_shared (rwx) : ORIGIN = 0x20007000, LENGTH =  4K
        ram_usb    (rwx) : ORIGIN = 0x20008000, LENGTH = 32K
        /* ram_usb: USB buffer. Straddles two blocks of RAM
         * to get performance benefit of having two USB buffers addressable
         * simultaneously (on two different buses of the AHB multilayer matrix)
         */
}
usb_bulk_buffer = ORIGIN(ram_usb);


/* Enforce emmission of the vector table. */
EXTERN (vector_table)
EXTERN (JumpTable)

/* Define the entry point of the output file. */
ENTRY(reset_handler)

EXTERN(__EXCEPTIONS);
EXTERN(__INTERRUPTS);
EXTERN(DefaultHandler);
PROVIDE(NonMaskableInt = DefaultHandler);
PROVIDE(MemoryManagement = DefaultHandler);
PROVIDE(BusFault = DefaultHandler);
PROVIDE(UsageFault = DefaultHandler);
PROVIDE(SecureFault = DefaultHandler);
PROVIDE(SVCall = DefaultHandler);
PROVIDE(DebugMonitor = DefaultHandler);
PROVIDE(PendSV = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);
PROVIDE(DAC = DefaultHandler);
PROVIDE(DMA = DefaultHandler);
PROVIDE(FLASH = DefaultHandler);
PROVIDE(ETHERNET = DefaultHandler);
PROVIDE(SDIO = DefaultHandler);
PROVIDE(LCD = DefaultHandler);
PROVIDE(USB0 = DefaultHandler);
PROVIDE(USB1 = DefaultHandler);
PROVIDE(SCT = DefaultHandler);
PROVIDE(RITIMER = DefaultHandler);
PROVIDE(TIMER0 = DefaultHandler);
PROVIDE(TIMER1 = DefaultHandler);
PROVIDE(TIMER2 = DefaultHandler);
PROVIDE(TIMER3 = DefaultHandler);
PROVIDE(MCPWM = DefaultHandler);
PROVIDE(ADC0 = DefaultHandler);
PROVIDE(I2C0 = DefaultHandler);
PROVIDE(I2C1 = DefaultHandler);
PROVIDE(SPI_INT = DefaultHandler);
PROVIDE(ADC1 = DefaultHandler);
PROVIDE(SSP0 = DefaultHandler);
PROVIDE(SSP1 = DefaultHandler);
PROVIDE(USART0 = DefaultHandler);
PROVIDE(UART1 = DefaultHandler);
PROVIDE(USART2 = DefaultHandler);
PROVIDE(USART3 = DefaultHandler);
PROVIDE(I2S0 = DefaultHandler);
PROVIDE(I2S1 = DefaultHandler);
PROVIDE(SPIFI = DefaultHandler);
PROVIDE(SGPIO_IINT = DefaultHandler);
PROVIDE(PIN_INT0 = DefaultHandler);
PROVIDE(PIN_INT1 = DefaultHandler);
PROVIDE(PIN_INT2 = DefaultHandler);
PROVIDE(PIN_INT3 = DefaultHandler);
PROVIDE(PIN_INT4 = DefaultHandler);
PROVIDE(PIN_INT5 = DefaultHandler);
PROVIDE(PIN_INT6 = DefaultHandler);
PROVIDE(PIN_INT7 = DefaultHandler);
PROVIDE(GINT0 = DefaultHandler);
PROVIDE(GINT1 = DefaultHandler);
PROVIDE(EVENTROUTER = DefaultHandler);
PROVIDE(C_CAN1 = DefaultHandler);
PROVIDE(ADCHS = DefaultHandler);
PROVIDE(ATIMER = DefaultHandler);
PROVIDE(RTC = DefaultHandler);
PROVIDE(WWDT = DefaultHandler);
PROVIDE(C_CAN0 = DefaultHandler);
PROVIDE(QEI = DefaultHandler);


SECTIONS
{
  /* ### Vector table */
  .vector_table ORIGIN(ram_local1) : ALIGN(4)
  {
    . = ALIGN(0x400);
    /* Initial Stack Pointer (SP) value */
    __STACK_START = .; /* Just to get a nicer name in the disassembly */
    LONG(_stack);

    __RESET_VECTOR = .; /* Just to get a nicer name in the disassembly */
    /* Reset vector */
    LONG(reset_handler);
    __reset_vector = ABSOLUTE(.);

    __EXCEPTIONS = .; /* Just to get a nicer name in the disassembly */
    /* Exceptions */
    KEEP(*(.vector_table.exceptions)); /* this is `__EXCEPTIONS` symbol */
    __eexceptions = ABSOLUTE(.);

    __INTERRUPTS = .; /* Just to get a nicer name in the disassembly */
    /* Device specific interrupts */
    KEEP(*(.vector_table.interrupts)); /* this is `__INTERRUPTS` symbol */
    __einterrupts = ABSOLUTE(.);
  } > ram_local1

  .jump : {
    KEEP(*(.jump))  /* Jump table */
  } > ram_local1

  .text : ALIGN(4)
  {
    _stext = ABSOLUTE(.);
    *(.text .text.*);
    _etext = ABSOLUTE(.);
    . = ALIGN(4);
    *(.rodata*);
  } > ram_local1

  . = ORIGIN(ram_local2);
  .data :
  {
    . = ALIGN(4);
    __sdata = ABSOLUTE(.);
    *(.data .data.*)
    . = ALIGN(4);
    __edata = (.);
  } > ram_local2 AT > ram_local1
  .bss :
  {
    . = ALIGN(4);
    /* force zero initialized data to be present*/
    __sbss = ABSOLUTE(.);
    *(.bss .bss.*)
    *(COMMON)
    . = ALIGN(4);
    __ebss = ABSOLUTE(.);
  } > ram_local2

  end = .;

  /* Leave room above stack for IAP to run. */
  __StackTop = ORIGIN(ram_local2) + LENGTH(ram_local2) - 32;
  PROVIDE(_stack = __StackTop);
  _l0dable_start = ORIGIN(ram_l0dable);
  _l0dable_len = LENGTH(ram_l0dable);
  /* _jumptable_len = SIZEOF(.jump); */

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx.*);
  }
}

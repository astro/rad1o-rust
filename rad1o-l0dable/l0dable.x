MEMORY
{
    shadow     (rx)  : ORIGIN = 0x00000000, LENGTH = 128K
    ram_l0dable(rwx) : ORIGIN = 0x10080000, LENGTH = 16K
}

/* ENTRY(start); */

SECTIONS
{
  .hack (NOLOAD) : { /* hack to advance ">ram_local2" to the current offset */
    KEEP(*(.jump))
    _jumpsize = .;
  } > shadow

  .myhdr : {
    LONG(_jumpsize);
    LONG(start);
  } > shadow

  .text : ALIGN(4)
  {
    _stext = ABSOLUTE(.);
    *(.text .text.*);
    _etext = ABSOLUTE(.);
    . = ALIGN(4);
    *(.rodata*);
  } > ram_l0dable AT > shadow
    
  .data :
  {
    . = ALIGN(4);
    __sdata = ABSOLUTE(.);
    *(.data .data.*)
    . = ALIGN(4);
    __edata = (.);
  } > ram_l0dable AT > shadow
  .bss :
  {
    . = ALIGN(4);
    /* force zero initialized data to be present*/
    __sbss = ABSOLUTE(.);
    *(.bss .bss.*)
    . = ALIGN(4);
    __ebss = ABSOLUTE(.);
  } > ram_l0dable AT > shadow

  end = .;

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx.*);
  }
}

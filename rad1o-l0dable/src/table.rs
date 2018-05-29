use core::mem::size_of;

/// Firmware call table linked to `0x0000_0124`
#[allow(non_snake_case, missing_docs)]
#[repr(C)]
pub struct Table {
    pub identifier: usize,
    pub delayNop: extern "C" fn(u32),
    pub getInput: extern "C" fn() -> u8,
    pub getInputRaw: extern "C" fn() -> u8,
    pub getInputWait: extern "C" fn() -> u8,
    pub getInputWaitRelease: extern "C" fn(),
    pub gpio_clear: extern "C" fn(u32, u32),
    pub gpio_set: extern "C" fn(u32, u32),
    pub gpio_toggle: extern "C" fn(u32, u32),
    pub IntToStr: extern "C" fn(isize, usize, u8) -> *const u8,
    pub lcdDisplay: extern "C" fn(),
    pub lcdNl: extern "C" fn(),
    pub lcdPrint: extern "C" fn(*const u8),
    pub lcdPrintln: extern "C" fn(*const u8),
    pub lcdSetCrsr: extern "C" fn(isize, isize),
    pub _timectr: *mut u32,
    pub lcdFill: extern "C" fn(u8),
    pub getInputWaitTimeout: extern "C" fn(i32) -> u8,
    pub font_dev: *mut (),
    pub setIntFont: extern "C" fn(*const ()),
    pub lcdSetPixel: extern "C" fn(u8, u8, u8),
    pub DoChar: extern "C" fn(isize, isize, isize) -> isize,
    pub setTextColor: extern "C" fn(u8, u8),
    pub lcdShift: extern "C" fn(isize, isize, isize),
    nickname: *mut u8,
    nickfont: *mut u8,
    pub lcdClear: extern "C" fn(),
    pub setExtFont: extern "C" fn(*const u8),
    pub DoString: extern "C" fn(isize, isize, *const u8) -> isize,
    pub getFontHeight: extern "C" fn() -> isize,
    pub lcdGetPixel: extern "C" fn(u8, u8) -> u8,
    pub delayms: extern "C" fn(u32),
    pub delayms_queue: extern "C" fn(u32),
    pub getRandom: extern "C" fn() -> u32,
    pub delayms_queue_plus: extern "C" fn(u32, u8) -> u8,
    pub cdesc: *mut (),
    pub lcdShowAnim: extern "C" fn(*const u8) -> u8,
    pub lcdShowImageFile: extern "C" fn(*const u8) -> u8,
    pub f_read: extern "C" fn(),
    pub f_write: extern "C" fn(),
    pub f_open: extern "C" fn(),
    pub f_lseek: extern "C" fn(),
    pub lcd_select: extern "C" fn(),
    pub lcd_deselect: extern "C" fn(),
    pub lcdWrite: extern "C" fn(u8, u8),
    pub ws2812_sendarray: extern "C" fn(u8, isize),
    pub batteryCharging: extern "C" fn() -> bool,
    pub batteryGetVoltage: extern "C" fn() -> u32,
    pub drawHLine: extern "C" fn(isize, isize, isize, u8),
    pub drawVLine: extern "C" fn(isize, isize, isize, u8),
    pub drawRectFill: extern "C" fn(isize, isize, isize, isize, u8),
    pub readTextFile: extern "C" fn(*const u8, *const u8, isize) -> isize,
    pub writeFile: extern "C" fn(*const u8, *const (), isize) -> isize,
    pub drawLine: extern "C" fn(isize, isize, isize, isize, u8, isize),
    pub fsInfo: extern "C" fn(*const ()) -> isize,
    pub fsUsage: extern "C" fn(*const (), *const ()) -> isize,
    pub lcdGetCrsrY: extern "C" fn() -> isize,
    pub lcdMoveCrsr: extern "C" fn(isize, isize),
    pub DoRect: extern "C" fn(isize, isize, isize, isize),
    pub DoLine: extern "C" fn(isize, isize, isize, isize),
    pub DoCube: extern "C" fn(*const isize, isize, *const f32),
    pub DoMesh: extern "C" fn(*const f32, isize, *const isize, isize, *const f32, *const isize, isize),
    pub getMeshSizes: extern "C" fn(*const u8, *const isize, *const isize) -> u8,
    pub getMesh: extern "C" fn(*const u8, *const f32, isize, *const isize, isize) -> u8,
    pub readFile: extern "C" fn(*const u8, *const (), isize) -> isize,
    pub work_queue: extern "C" fn(),
}

const JUMP_TABLE_SIZE: usize = size_of::<Table>();

#[no_mangle]
#[link_section = ".jump"]
pub static JumpTable: [u8; JUMP_TABLE_SIZE] = [0u8; JUMP_TABLE_SIZE];

// #[no_mangle]
// #[link_section = ".jump"]
// pub static JumpTable: Table = unsafe { *null() };

/// Return safe pointer to `JumpTable`
pub fn table() -> &'static Table {
    let jump_table = 0x10000114 as *const Table;
    unsafe { &*jump_table }
}

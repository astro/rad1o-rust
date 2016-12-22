use core::fmt::Arguments;

// #[cfg(feature = "default-panic-fmt")]
#[lang = "panic_fmt"]
#[export_name = "rust_begin_unwind"]
pub unsafe fn panic_fmt(_msg: Arguments,
                        _file: &'static str,
                        _line: u32)
                        -> ! {
    loop {}
}

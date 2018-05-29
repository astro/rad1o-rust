use r0;

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub extern "C" fn __impl_main() {
            let f: fn() = $path;
            f()
        }
    }
}

/// Start-up
///
/// * Zeroes the `.bss` segment
/// * Calls your main function which must be set up using the `entry!()` macro.
///
/// Gets linked in first by the `l0dable.x` linker script.
#[no_mangle]
pub extern "C" fn start() {
    extern "C" {
        static mut __sbss: u32;
        static mut __ebss: u32;
        fn main();
    }
    unsafe {
        // Initialize RAM
        r0::zero_bss(&mut __sbss, &mut __ebss);

        // Call __impl_main()
        main();
    }
}

#![no_std]
#![feature(global_asm)]

extern crate panic_abort;

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            let f: fn() -> ! = $path;
            f()
        }
    };
}


/// Reset function
/// Initialize the bss section before calling into the user's main
#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static mut __bss_start: u64;
        static mut __bss_end: u64;
    }
    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    extern "Rust" {
        fn main() -> !;
    }
    main();
}

global_asm!(include_str!("boot.S"));
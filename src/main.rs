#![no_std]
#![no_main]

fn kernel_entry() -> ! {
    loop {}
}

boot::entry!(kernel_entry);
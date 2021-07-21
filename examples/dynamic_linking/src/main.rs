#![no_std]
#![no_main]

#[no_mangle]
pub unsafe extern "C" fn _start(_args: i32, _argp: *const u8) -> ! {
    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

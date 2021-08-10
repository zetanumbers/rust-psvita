#![no_std]
#![no_main]

#[no_mangle]
pub unsafe extern "C" fn _start(_args: i32, _argp: *const u8) -> ! {
    while psvita_dylib_example::foo() {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

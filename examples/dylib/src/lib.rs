#![no_std]

pub fn foo() -> i32 {
    42
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#![no_std]
#![no_main]

mod types {
    #![allow(non_camel_case_types)]

    pub type c_int = i32;
    pub type c_char = u8;
    pub type SceUInt = u32;
}
use types::*;

#[link(name = "SceLibKernel_stub")]
extern "C" {
    fn sceKernelExitProcess(exit_code: c_int) -> c_int;
}

#[link(name = "SceKernelThreadMgr_stub")]
extern "C" {
    fn sceKernelDelayThread(delay: SceUInt) -> c_int;
}

#[no_mangle]
#[used]
static test_data: [u8; 128] = [0; 128];

#[no_mangle]
pub unsafe extern "C" fn _start(_args: c_int, _argp: *const c_char) -> ! {
    let _ = sceKernelDelayThread(2_000_000);
    let _ = sceKernelExitProcess(0);
    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

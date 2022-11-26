#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use hi_os::{self, QemuExitCode, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    hi_os::exit_qemu(QemuExitCode::Failure);
    loop {}
}

fn should_fail() {
    hi_os::serial_print!("should_fail... ");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    hi_os::exit_qemu(QemuExitCode::Success);
    loop {}
}

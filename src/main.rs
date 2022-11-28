#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hi_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use hi_os::println;

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    hi_os::init();
    
    #[cfg(test)]
    test_main();

    println!("Succeeded");
    hi_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    hi_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hi_os::test_panic_handler(info)
}

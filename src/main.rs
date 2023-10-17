#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::{println, hlt_loop};

#[no_mangle]
pub extern "C" fn _start() -> ! {

    blog_os::init();

    blog_os::display::line();
    println!("RustOS 0.0.1,Welcome!");
    blog_os::display::line();
    println!("\n");


    #[cfg(test)]
    test_main();

    hlt_loop();
}



/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
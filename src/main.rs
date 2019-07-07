#![feature(lang_items)]
#![no_std]
#![no_main]
#![windows_subsystem = "console"]   // set Entrypoint to "mainCRTStartup"

use core::panic::PanicInfo;

use winapi::um::processthreadsapi::ExitProcess;
use winapi::um::fileapi::WriteFile;

mod bumsti;

#[panic_handler]            #[no_mangle] pub extern fn panic(_info: &PanicInfo) -> ! { loop {} }
#[lang = "eh_personality"]  #[no_mangle] pub extern fn eh_personality() {}

#[no_mangle] // don't mangle the name of this function
pub extern fn mainCRTStartup() -> ! {

    let rc = bumsti::write_stdout("hello berni!");

    unsafe { ExitProcess(128); }
    loop {}
}

// Pull in the system libc library for what crt0.o likely requires.
//extern crate libc;

/*
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}
*/
/*
#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    0
}
*/
/*
#[entry]
#[no_mangle]
pub extern "system" fn mainCRTStartup()  {
    unsafe
        {
            winapi::um::processthreadsapi::ExitProcess(99);
        }
}
*/


/*
#[start]
fn my_start(argc: isize, argv: *const *const u8) -> isize {
    0
}

#[main]
fn main() {

}

fn ixi() {
    //bumsti::doTheExit(99);
}
*/


/*
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
*/

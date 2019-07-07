#![feature(lang_items)]
#![no_std]
#![no_main]
#![windows_subsystem = "console"]   // set Entrypoint to "mainCRTStartup"

use core::panic::PanicInfo;

use winapi::um::processthreadsapi::ExitProcess;

mod bumsti;

#[panic_handler]            #[no_mangle] pub extern fn panic(_info: &PanicInfo) -> ! { loop {} }
#[lang = "eh_personality"]  #[no_mangle] pub extern fn eh_personality() {}

#[no_mangle] // don't mangle the name of this function
pub extern fn mainCRTStartup() -> ! {

    let rc = bumsti::write_stdout("höllo berni!");

    unsafe { ExitProcess(128); }
    loop {}
}

#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![no_std]
#![no_main]
#![windows_subsystem = "console"]   // set Entrypoint to "mainCRTStartup"

extern crate alloc;

pub mod win32alloc;
#[global_allocator]
static BEE_NO_STD_ALLOCATOR: Heapalloc = win32alloc::Heapalloc;

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

use core::panic::PanicInfo;

use alloc::vec::{Vec};

use winapi::um::processthreadsapi::ExitProcess;
use winapi::um::shellapi::CommandLineToArgvW;
use winapi::um::processenv::GetCommandLineW;
use winapi::um::winbase::LocalFree;
use winapi::um::winnt::LPWSTR;
use winapi::shared::minwindef::HLOCAL;
use crate::win32alloc::Heapalloc;

mod bumsti;

#[panic_handler]
#[no_mangle]
pub extern fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[no_mangle] // don't mangle the name of this function
pub extern fn mainCRTStartup() {

    //let rc = bumsti::write_stdout("höllo berni!");

    unsafe {
        let mut argc : i32 = 0;
        let argv:*mut LPWSTR = CommandLineToArgvW(GetCommandLineW(), &mut argc);

        let beeMainExitCode = beeMain(argc, argv);

        LocalFree(argv as HLOCAL);
        ExitProcess(beeMainExitCode);
    }
    //loop {}
}

fn beeMain(argc : i32, argv : *mut LPWSTR) -> u32 {
    //let first = argv as &[u16];

    let mut first_veccal : Vec<u16> = Vec::new();

    first_veccal.push(128);

    argc as u32
}
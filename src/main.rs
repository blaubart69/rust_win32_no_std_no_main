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
    //panic!("allocation error: {:?}", layout)
    loop {}
}

//#[no_mangle]
//pub extern "C" fn __CxxFrameHandler3() {
//}
/*
#[no_mangle]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8,
                            n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    return dest;
}

#[no_mangle]
pub unsafe extern fn memmove(dest: *mut u8, src: *const u8,
                             n: usize) -> *mut u8 {
    if src < dest as *const u8 { // copy from end
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else { // copy from beginning
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }
    return dest;
}

#[no_mangle]
pub unsafe extern fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    return s;
}

#[no_mangle]
pub unsafe extern fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32
        }
        i += 1;
    }
    return 0;
}
*/

#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

use core::panic::PanicInfo;

use alloc::vec::{Vec};
use alloc::boxed::{Box};

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

        //let arg0 = alloc::string::String::from_utf16(*argv[0]).unwrap();
        //bumsti::write_stdout(arg0.as_str());

        let beeMainExitCode = beeMain(argc, argv);

        LocalFree(argv as HLOCAL);
        ExitProcess(beeMainExitCode);
    }
}

fn beeMain(argc : i32, argv : *mut LPWSTR) -> u32 {
    //let first = argv as &[u16];

    let mut first_veccal : Vec<u16> = Vec::new();

    first_veccal.push(128);

    let boxxi = Box::new(5);
    let rc = t1(&first_veccal, &boxxi);
    rc as u32
}

fn t1(v : &Vec<u16>, b : &Box<i32>) -> i32
{
    let v_sum : u16 = v.iter().sum();
    let s_vale : i32 = **b;
    v_sum as i32 + s_vale
}
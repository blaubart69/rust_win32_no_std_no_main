use winapi::um::fileapi::WriteFile;
use winapi::shared::minwindef::{UINT, DWORD, BOOL};
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::winnt::HANDLE;
use winapi::um::minwinbase::LPOVERLAPPED;

use core::str;

pub fn write_stdout(text: &str) -> BOOL {
    unsafe {
        let stdout: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);

        let mut written : DWORD;
        //let mut overlapped : LPOVERLAPPED;
        let *mut overlapped : LPOVERLAPPED = core::ptr::null();

        let ok = WriteFile(
            stdout,
            "öjlöj",
            3,
            &mut written,
            mut overlapped
        );

        ok
    }
}
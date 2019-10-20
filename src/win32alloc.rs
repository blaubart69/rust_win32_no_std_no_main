use alloc::alloc::{GlobalAlloc, Layout};

pub struct Heapalloc;

unsafe impl GlobalAlloc for Heapalloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {

        winapi::um::heapapi::HeapAlloc(
            winapi::um::heapapi::GetProcessHeap(),
            0,
            _layout.size()) as *mut u8

    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        winapi::um::heapapi::HeapAlloc(
            winapi::um::heapapi::GetProcessHeap(),
            winapi::um::winnt::HEAP_ZERO_MEMORY,
            layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {

        winapi::um::heapapi::HeapFree(
            winapi::um::heapapi::GetProcessHeap(),
            0,
            _ptr as *mut winapi::ctypes::c_void) ;

    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        winapi::um::heapapi::HeapReAlloc(
            winapi::um::heapapi::GetProcessHeap(),
            winapi::um::winnt::HEAP_ZERO_MEMORY,
            ptr as *mut winapi::ctypes::c_void,
            new_size)
        as *mut u8
    }
}
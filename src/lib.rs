#![no_std]
#![allow(non_snake_case)]
#![feature(alloc)]
#![feature(alloc_prelude)]

extern crate alloc;
extern crate win_kmd_alloc;
extern crate win_kmd_native_lib;

use crate::win_kmd_alloc::KernelAlloc;
use winapi::{
    shared::{
        ntstatus::*,
        ntdef::*,
    },
};
use core::panic::PanicInfo;

#[global_allocator]
static GLOBAL: KernelAlloc = KernelAlloc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "system" fn DriverEntry(_driver: PVOID, _path: PVOID) -> NTSTATUS {
    STATUS_SUCCESS
}

#![no_std]
#![no_main]
// #![feature(alloc_error_handler)]
#![feature(panic_info_message)]
extern crate MatiOS_SDK;
extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec;
use core::cell::RefCell;
use core::ffi::c_void;
use MatiOS_SDK::uuid::Uuid;

#[unsafe(no_mangle)]
pub extern "win64" fn _start() -> ! {
    unsafe {
        main(999 as *mut ProcessStartInfo);
    }
    loop {}
}
#[unsafe(link_section = ".main")]
#[unsafe(no_mangle)]
pub unsafe extern "win64" fn main(a: *const ProcessStartInfo) -> u64 {
    ALLOC.init((*a).allocate);
    ProcessStartInfo::init(a);

    let std_out: *const CreateResourceV1Response = syscall_sync(
        syscall_id::CREATE_RESOURCE_V1,
        CreateResourceV1 {
            resource_type: RESOURCE_BYTE_STREAM_ID,
            owner: ProcessStartInfo::get().processId,
            tags: vec![RESOURCE_BYTE_STREAM_TAG_STDOUT],
        },
    );

    syscall_sync_noreturn(
        syscall_id::CALL_RESOURCE_METHOD_V1,
        CallResourceMethodV1 {
            resource: (*std_out).uuid,
            method: String::from("write"),
            args: TypedValue::string(String::from("Hello, World!")),
        },
    );
    return 0;
}
use core::panic::PanicInfo;
use MatiOS_SDK::allocator::LinkedListAlloc;
use MatiOS_SDK::gui::bitmap::Color;
use MatiOS_SDK::gui::controls::flex::FlexControl;
use MatiOS_SDK::gui::controls::text::TextControl;
use MatiOS_SDK::gui::controls::Control;
use MatiOS_SDK::process_start_info::{syscall_sync, syscall_sync_noreturn, ProcessStartInfo};
use MatiOS_SDK::resources::{RESOURCE_BYTE_STREAM_ID, RESOURCE_BYTE_STREAM_TAG_STDOUT, RESOURCE_DESKTOP_ID, RESOURCE_WINDOW_ID};
use MatiOS_SDK::syscalls::debug::print_v1::PrintV1;
use MatiOS_SDK::syscalls::process::current_process_info_v1::{
    CurrentProcessInfoV1Request, CurrentProcessInfoV1Response,
};
use MatiOS_SDK::syscalls::resources::call_resource_method_v1::{
    CallResourceMethodV1, CallResourceMethodV1Response,
};
use MatiOS_SDK::syscalls::resources::create_resource_v1::{
    CreateResourceV1, CreateResourceV1Response,
};
use MatiOS_SDK::syscalls::resources::request_resource_v1::{RequestResourceV1, RequestResourceV1Response};
use MatiOS_SDK::syscalls::{syscall_id, SyscallResponse};
use MatiOS_SDK::typed_value::{KeyedTypedValue, TypedValue};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if ProcessStartInfo::get() as *const ProcessStartInfo != 0 as *const ProcessStartInfo {
        (ProcessStartInfo::get().debugPrint)("Panic from inside demo app!");
        (ProcessStartInfo::get().debugPanicRust)(_info);
        (ProcessStartInfo::get().debugPrintInt)(_info.location().unwrap().line() as u64);
        (ProcessStartInfo::get().debugPrintInt)(_info.location().unwrap().file().len() as u64);
        let message = _info.message();

        (ProcessStartInfo::get().debugPrint)(message.as_str().unwrap());
    }

    loop {}
}

#[global_allocator]
pub static ALLOC: LinkedListAlloc = LinkedListAlloc::new();

#[unsafe(no_mangle)]
pub extern "C" fn memcpy(
    dest_cvoid: *mut c_void,
    src_cvoid: *const c_void,
    n: usize,
) -> *mut c_void {
    unsafe {
        let dest = dest_cvoid as *mut u8;
        let src = src_cvoid as *const u8;
        for i in 0..n {
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
        dest as *mut c_void
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn memset(dest_cvoid: *mut c_void, val_i32: i32, n: usize) -> *mut c_void {
    unsafe {
        let dest = dest_cvoid as *mut u8;
        let val = val_i32 as u8;
        for i in 0..n {
            *dest.offset(i as isize) = val;
        }
        dest_cvoid
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn memcmp(s1_cvoid: *const c_void, s2_cvoid: *const c_void, n: usize) -> i32 {
    unsafe {
        let s1 = s1_cvoid as *const u8;
        let s2 = s2_cvoid as *const u8;
        for i in 0..n {
            if *s1.offset(i as isize) != *s2.offset(i as isize) {
                return 1;
            }
        }
    }
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn memmove(
    dest_cvoid: *mut c_void,
    src_cvoid: *const c_void,
    n: usize,
) -> *mut c_void {
    unsafe {
        let dest = dest_cvoid as *mut u8;
        let src = src_cvoid as *const u8;
        if dest < src as *mut u8 {
            for i in 0..n {
                *dest.offset(i as isize) = *src.offset(i as isize);
            }
        } else {
            for i in (0..n).rev() {
                *dest.offset(i as isize) = *src.offset(i as isize);
            }
        }
        dest_cvoid
    }
}
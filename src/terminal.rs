use alloc::string::String;
use alloc::vec;
use MatiOS_SDK::process_start_info::{syscall_sync, syscall_sync_noreturn, ProcessStartInfo};
use MatiOS_SDK::resources::{RESOURCE_BYTE_STREAM_ID, RESOURCE_BYTE_STREAM_TAG_STDIN, RESOURCE_BYTE_STREAM_TAG_STDOUT};
use MatiOS_SDK::syscalls::resources::call_resource_method_v1::{CallResourceMethodV1, CallResourceMethodV1Response};
use MatiOS_SDK::syscalls::resources::create_resource_v1::{CreateResourceV1, CreateResourceV1Response};
use MatiOS_SDK::syscalls::syscall_id;
use MatiOS_SDK::typed_value::TypedValue;

static mut GLOBAL_STD_IN: *const CreateResourceV1Response = 0 as *const CreateResourceV1Response;
static mut GLOBAL_STD_OUT: *const CreateResourceV1Response = 0 as *const CreateResourceV1Response;

pub struct Terminal;
impl Terminal {
    pub fn init() {
        unsafe {
            GLOBAL_STD_OUT = syscall_sync(
                syscall_id::CREATE_RESOURCE_V1,
                CreateResourceV1 {
                    resource_type: RESOURCE_BYTE_STREAM_ID,
                    owner: ProcessStartInfo::get().processId,
                    tags: vec![RESOURCE_BYTE_STREAM_TAG_STDOUT],
                },
            );
            GLOBAL_STD_IN = syscall_sync(
                syscall_id::CREATE_RESOURCE_V1,
                CreateResourceV1 {
                    resource_type: RESOURCE_BYTE_STREAM_ID,
                    owner: ProcessStartInfo::get().processId,
                    tags: vec![RESOURCE_BYTE_STREAM_TAG_STDIN],
                },
            );
        }
    }
    pub fn read() -> String {
        unsafe {
            let read_result: *const CallResourceMethodV1Response = syscall_sync(
                syscall_id::CALL_RESOURCE_METHOD_V1,
                CallResourceMethodV1 {
                    resource: (*GLOBAL_STD_IN).uuid,
                    method: String::from("read"),
                    args: TypedValue::null(),
                },
            );

            let read = unsafe { &(*read_result).value.to_string_verbose() };
            return read.clone();
        }
    }
    pub fn write(data: &str) {
        unsafe {
            syscall_sync_noreturn(
                syscall_id::CALL_RESOURCE_METHOD_V1,
                CallResourceMethodV1 {
                    resource: (*GLOBAL_STD_OUT).uuid,
                    method: String::from("write"),
                    args: TypedValue::string(String::from(data)),
                },
            );
        }
    }
}
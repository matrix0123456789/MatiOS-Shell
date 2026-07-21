use crate::commands::Command;
use MatiOS_SDK::process_start_info::syscall_sync;
use MatiOS_SDK::syscalls::resources::call_resource_method_v1::{
    CallResourceMethodV1, CallResourceMethodV1Response,
};
use MatiOS_SDK::syscalls::resources::get_resource_by_path_v1::{
    GetResourceByPathV1Request, GetResourceByPathV1Response,
};
use MatiOS_SDK::syscalls::resources::get_resource_info_v1::{
    GetResourceInfoV1Request, GetResourceInfoV1Response,
};
use MatiOS_SDK::syscalls::syscall_id::{
    CALL_RESOURCE_METHOD_V1, GET_RESOURCE_BY_PATH_V1, GET_RESOURCE_INFO_V1,
};
use MatiOS_SDK::uuid::Uuid;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use MatiOS_SDK::typed_value::TypedValue;

pub struct LsCommand {}
impl Command for LsCommand {
    fn execute(&self, args: Vec<(String, String)>) {
        let tmp_uuid = Uuid::from_u128(0x200000002);
        // let info: *const GetResourceInfoV1Response =
        //     syscall_sync(GET_RESOURCE_INFO_V1, GetResourceInfoV1Request { uuid: tmp_uuid });
        let children: *const CallResourceMethodV1Response = syscall_sync(
            CALL_RESOURCE_METHOD_V1,
            CallResourceMethodV1 {
                resource: tmp_uuid,
                method: String::from("getChildren"),
                args: TypedValue::null(),
            },
        );
        for x in unsafe { (*children).value.to_vec() } {
            crate::terminal::Terminal::write(x.to_string_verbose().as_str());
            crate::terminal::Terminal::write("\n");
        }
    }
    fn get_name(&self) -> String {
        String::from("ls")
    }
    fn get_aliases(&self) -> Vec<String> {
        vec![String::from("dir")]
    }
    fn get_help(&self) -> String {
        String::from("Lists files and directories")
    }
    fn get_params(&self) -> crate::commands::ParamsDefinition {
        crate::commands::ParamsDefinition {
            params: vec![],
            ordered_params: vec![],
            subcommands: vec![],
        }
    }
}

use crate::commands::Command;
use MatiOS_SDK::process_start_info::syscall_sync;
use MatiOS_SDK::syscalls::resources::get_resource_by_path_v1::{
    GetResourceByPathV1Request, GetResourceByPathV1Response,
};
use MatiOS_SDK::syscalls::resources::get_resource_info_v1::{
    GetResourceInfoV1Request, GetResourceInfoV1Response,
};
use MatiOS_SDK::syscalls::syscall_id::{GET_RESOURCE_BY_PATH_V1, GET_RESOURCE_INFO_V1};
use MatiOS_SDK::uuid::Uuid;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

pub struct ResourceCommand {}
impl Command for ResourceCommand {
    fn execute(&self, args: Vec<(String, String)>) {
        let path = args.get(0).unwrap().1.as_str();
        let mut uuidR = Uuid::parse_str(path);
        let uuid;
        if uuidR.is_err() {
            let uuidResponse: *const GetResourceByPathV1Response = syscall_sync(
                GET_RESOURCE_BY_PATH_V1,
                GetResourceByPathV1Request {
                    path: String::from(path),
                },
            );
            uuid = unsafe { (*uuidResponse).uuid };
        } else {
            uuid = uuidR.unwrap();
        }
        let info: *const GetResourceInfoV1Response =
            syscall_sync(GET_RESOURCE_INFO_V1, GetResourceInfoV1Request { uuid });
        if(unsafe{(*info).uuid.as_u128()} == 0) {
            crate::terminal::Terminal::write("Resource not found\n");
            return;
        }
        unsafe {
            crate::terminal::Terminal::write("Name:");
            crate::terminal::Terminal::write((*info).name.as_str());
            crate::terminal::Terminal::write("\nUuid:");
            crate::terminal::Terminal::write((*info).uuid.to_string().as_str());
            crate::terminal::Terminal::write("\nUuid:");
            crate::terminal::Terminal::write("\n<h2>Connected resources:</h2>\n");
            crate::terminal::Terminal::write("<list>\n");
            for x in unsafe { (*info).connected_resources.iter() } {
                crate::terminal::Terminal::write("<object>");
                crate::terminal::Terminal::write("<value name=\"uuid\">");
                crate::terminal::Terminal::write(x.to_string().as_str());
                crate::terminal::Terminal::write("</value>");
                crate::terminal::Terminal::write("</object>");
                crate::terminal::Terminal::write("\n");
            }
            crate::terminal::Terminal::write("</list>");
        }
        crate::terminal::Terminal::write("\n<h2>Methods:</h2>\n");
        crate::terminal::Terminal::write("<list>\n");
        for x in unsafe { (*info).methods.iter() } {
            crate::terminal::Terminal::write("<object>");
            crate::terminal::Terminal::write("<value name=\"name\">");
            crate::terminal::Terminal::write(x.as_str());
            crate::terminal::Terminal::write("</value>");
            crate::terminal::Terminal::write("</object>");
            crate::terminal::Terminal::write("\n");
        }
        crate::terminal::Terminal::write("</list>");
    }
    fn get_name(&self) -> String {
        String::from("resource")
    }
    fn get_aliases(&self) -> Vec<String> {
        vec![String::from("res")]
    }
    fn get_help(&self) -> String {
        String::from("Manages resources")
    }
    fn get_params(&self) -> crate::commands::ParamsDefinition {
        crate::commands::ParamsDefinition {
            params: vec![],
            ordered_params: vec![],
            subcommands: vec![],
        }
    }
}

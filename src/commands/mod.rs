use alloc::string::String;
use alloc::vec::Vec;

pub mod resource_command;
pub mod help_command;

pub trait Command: Sync {
    fn execute(&self);
    fn get_name(&self) ->String;
    fn get_aliases(&self) ->Vec<String>;
    fn get_help(&self) ->String;
}
static RESOURCE_COMMAND: resource_command::ResourceCommand = resource_command::ResourceCommand {};
static HELP_COMMAND: help_command::HelpCommand = help_command::HelpCommand {};

pub static BUILT_IN_COMMANDS: [&dyn Command; 2] = [&RESOURCE_COMMAND, &HELP_COMMAND];

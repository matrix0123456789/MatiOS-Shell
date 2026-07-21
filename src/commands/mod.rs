use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

pub mod resource_command;
pub mod help_command;
pub mod ls_command;

pub trait Command: Sync {
    fn execute(&self, args: Vec<(String, String)>);
    fn get_name(&self) ->String;
    fn get_aliases(&self) ->Vec<String>;
    fn get_help(&self) ->String;
    fn get_params(&self) ->ParamsDefinition;
}
static RESOURCE_COMMAND: resource_command::ResourceCommand = resource_command::ResourceCommand {};
static HELP_COMMAND: help_command::HelpCommand = help_command::HelpCommand {};
static LS_COMMAND: ls_command::LsCommand = ls_command::LsCommand {};

pub static BUILT_IN_COMMANDS: [&dyn Command; 3] = [&RESOURCE_COMMAND, &HELP_COMMAND, &LS_COMMAND];

pub struct ParamsDefinition{
    pub params: Vec<ParamDefinition>,
    pub ordered_params: Vec<String>,
    pub subcommands:Vec<Box<dyn Command>>
}
pub struct ParamDefinition{
    pub name: String,
    pub title: String,
    pub description: String,

}
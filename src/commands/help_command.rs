use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use crate::commands::{Command, BUILT_IN_COMMANDS};

pub struct HelpCommand {
}
impl Command for HelpCommand{
    fn execute(&self, args: Vec<(String, String)>) {
        let command;
        if(args.len() == 0){
            command = "help";
        }
        else {
            command = args.get(0).unwrap().1.as_str();
        }
        let found_command = BUILT_IN_COMMANDS.iter().find(|c| {
            c.get_name() == command || c.get_aliases().iter().any(|alias| alias == command)
        });
        if(found_command.is_some()){
            crate::terminal::Terminal::write("<h1>");
            crate::terminal::Terminal::write(found_command.unwrap().get_name().as_str());
            crate::terminal::Terminal::write("</h1>\n");
            crate::terminal::Terminal::write(found_command.unwrap().get_help().as_str());
            crate::terminal::Terminal::write("\n\n<h2>Parameters</h2>\n");
            for x in found_command.unwrap().get_params().params.iter() {
                crate::terminal::Terminal::write(x.name.as_str());
                crate::terminal::Terminal::write("\n");
            }

        }else{
            crate::terminal::Terminal::write("Command not found");
        }

    }
    fn get_name(&self) -> String {
        String::from("help")
    }
    fn get_aliases(&self) -> Vec<String> {
        vec![String::from("man")]
    }
    fn get_help(&self) -> String {
        String::from("Shows help for a specific command")
    }
    fn get_params(&self) -> crate::commands::ParamsDefinition {
        crate::commands::ParamsDefinition { params: vec![
            crate::commands::ParamDefinition {
                name: String::from("command"),
                title: String::from("Command"),
                description: String::from("The command to show help for")
            }
        ], ordered_params: vec![String::from("command")], subcommands: vec![] }
    }
}
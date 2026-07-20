use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use crate::commands::Command;

pub struct HelpCommand {
}
impl Command for HelpCommand{
    fn execute(&self) {
        crate::terminal::Terminal::write("help command executed");
    }
    fn get_name(&self) -> String {
        String::from("help")
    }
    fn get_aliases(&self) -> Vec<String> {
        vec![String::from("man")]
    }
    fn get_help(&self) -> String {
        String::from("Help command help")
    }
}
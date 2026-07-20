use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use crate::commands::Command;

pub struct ResourceCommand{

}
impl Command for ResourceCommand {
    fn execute(&self) {
        crate::terminal::Terminal::write("res command executed");
    }
    fn get_name(&self) -> String {
        String::from("resource")
    }
    fn get_aliases(&self) -> Vec<String> {
        vec![String::from("res")]
    }
    fn get_help(&self) -> String {
        String::from("Resource command help")
    }
}
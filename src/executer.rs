use alloc::string::String;
use alloc::vec;
use crate::commands::BUILT_IN_COMMANDS;
use crate::terminal::Terminal;

pub struct Executer {

}
impl  Executer {
 pub fn ExecuteLine(line: String){
     let command = line.split_whitespace().next().unwrap();
     let found_command = BUILT_IN_COMMANDS.iter().find(|c| {
        c.get_name() == command || c.get_aliases().iter().any(|alias| alias == command)
     });
     if let Some(command) = found_command {
         let params = line.split_whitespace().skip(1).map(|s| (String::from("tmp"), String::from(s))).collect();
         command.execute(params);
     }else{
         Terminal::write("Command not found");
     }
 }
}
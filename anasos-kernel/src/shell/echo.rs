use alloc::string::String;

use super::Command;


pub(crate) struct Echo {}

impl Command for Echo {
    fn execute(&self, args: String) -> String {
        args
    }
    
}
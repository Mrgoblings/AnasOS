use alloc::{string::String, vec::Vec};

use super::Command;


pub(crate) struct Echo {}

impl Command for Echo {
    fn execute(&self, args: Vec<&str>) -> String {
        args.join(" ")
    }
    
}
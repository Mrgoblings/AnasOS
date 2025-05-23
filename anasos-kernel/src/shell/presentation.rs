use alloc::{string::String, vec::Vec};

use super::Command;


pub(crate) struct Presentation {}

impl Command for Presentation {
    fn execute(&self, args: Vec<&str>) -> String {
        args.join(" ")
    }
    
}
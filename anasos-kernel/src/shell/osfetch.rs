use alloc::{string::{String, ToString}, vec::Vec};

use super::Command;


pub(crate) struct OsFetch {}

impl Command for OsFetch {
    fn execute(&self, _args: Vec<&str>) -> String {
        "AnasOS 0.1.0\n".to_string()
    }
}

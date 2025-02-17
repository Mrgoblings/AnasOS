use alloc::string::{String, ToString};

use super::Command;


pub(crate) struct OsFetch {}

impl Command for OsFetch {
    fn execute(&self, _args: String) -> String {
        "AnasOS 0.1.0".to_string()
    }
}

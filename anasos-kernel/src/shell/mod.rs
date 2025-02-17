use alloc::{
    boxed::Box,
    collections::btree_map::BTreeMap,
    format,
    string::{String, ToString},
    vec::Vec,
};
use crossbeam_queue::ArrayQueue;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use x86_64::instructions::port::PortReadOnly;

use crate::{apps::terminal::BUFFER_SIZE, println};

mod echo;
mod osfetch;

pub trait Command {
    fn execute(&self, args: String) -> String;
}

/*
    ; Shell
    ; A simple shell that can execute commands
    ; Every shell has a prefix, which is the string that is printed before the user input
    ; The shell has a buffer of size BUFFER_SIZE, which is the maximum number of characters that can be typed
    ; The shell has a cursor, which is the current position in the buffer
    ; The shell has a start_last_command, which is the position in the buffer where the last command started
    ; The shell has a BTreeMap of commands, which maps a command name to a Command trait object
    ;    The shell has 2 default commands: { help } and { clear }, which use the shell directly
    ;    The shell has a method to add custom commands
    ; The shell has an input queue, which is used to store scancodes
*/
pub struct Shell {
    buffer: [char; BUFFER_SIZE],
    cursor: usize,
    start_last_command: usize,
    prefix: &'static str,
    commands: BTreeMap<&'static str, Box<dyn Command>>,

    // input queue
    scancode_queue: ArrayQueue<u8>,
    keyboard: Keyboard<layouts::Us104Key, ScancodeSet1>,

    history: Vec<String>,
}

impl Shell {
    pub fn new(prefix: &'static str) -> Self {
        let mut commands: BTreeMap<&str, Box<dyn Command>> = BTreeMap::new();
        // set custom commands
        commands.insert("echo", Box::new(echo::Echo {}));
        commands.insert("osfetch", Box::new(osfetch::OsFetch {}));

        Shell {
            buffer: ['\0'; BUFFER_SIZE],
            cursor: 0,
            start_last_command: 0,
            prefix,
            commands,
            scancode_queue: ArrayQueue::new(100),
            keyboard: Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore,
            ),
            history: Vec::new(),
        }
    }

    pub fn execute(&mut self, command: &str, args: String) -> String {
        let command = command.trim();
        println!("SHELL> Executing command: {}", command);
        self.history.push(command.to_string());

        if command == "help" {
            return self.command_help();
        } else if command == "clear" {
            return self.command_clear();
        }

        match self.commands.get(command) {
            Some(command) => command.execute(args),
            None => format!("{}: command not found", command),
        }
    }

    pub fn complete(&self, input: &str) -> String {
        let mut completions = self
            .commands
            .keys()
            .filter(|command| command.starts_with(input))
            .map(|command| command.to_string())
            .collect::<Vec<String>>();

        completions.sort();

        if completions.len() == 1 {
            completions.remove(0)
        } else {
            completions.join(" ")
        }
    }

    //input methods
    pub fn scancode_push(&self, scancode: u8) -> Result<(), ()> {
        println!("SHELL> scancode_push: scancode: {}", scancode);
        if let Err(_) = self.scancode_queue.push(scancode) {
            println!("SHELL> WARNING: Scancode queue full; dropping keyboard input");
            return Err(());
        }
        Ok(())
    }

    /*
        ; handle_input
        ; Handle the input from the keyboard
        ; The method pops a scancode from the queue and processes it
        ; If the scancode is a key event, the method processes the key event
        ; If the key event is a raw key, the method processes the raw key
        ; If the key event is a unicode character, the method processes the character
        ; The method returns a string, which is:
            - the output of the command  
            - possible completions
    */
    pub fn handle_input(&mut self) -> String {
        //keyboard input
        while let Some(scancode) = self.scancode_queue.pop() {
            if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
                if let Some(key) = self.keyboard.process_keyevent(key_event) {
                    println!("SHELL> Key: {:?}", key);
                    match key {
                        DecodedKey::RawKey(key_code) => {
                            println!("SHELL> Raw key: {:?}", key_code);
                            match key_code {
                                pc_keyboard::KeyCode::Backspace | pc_keyboard::KeyCode::Tab => {
                                    // never reached, unicode version handles them
                                    return String::new();
                                },
                                pc_keyboard::KeyCode::F1 | pc_keyboard::KeyCode::F2 | pc_keyboard::KeyCode::F3 | pc_keyboard::KeyCode::F4 | pc_keyboard::KeyCode::F5 | pc_keyboard::KeyCode::F6 | pc_keyboard::KeyCode::F7 | pc_keyboard::KeyCode::F8 | pc_keyboard::KeyCode::F9 | pc_keyboard::KeyCode::F10 | pc_keyboard::KeyCode::F11 | pc_keyboard::KeyCode::F12 => {
                                    return String::new();
                                },
                                pc_keyboard::KeyCode::ArrowUp => {
                                    // TODO: implement history

                                    return String::new();
                                },
                                pc_keyboard::KeyCode::ArrowDown => {
                                    // TODO: implement history

                                    return String::new();
                                },
                                pc_keyboard::KeyCode::ArrowLeft => {
                                    if self.cursor > 0 {
                                        self.cursor -= 1;
                                    }
                                    return String::new();
                                },
                                pc_keyboard::KeyCode::ArrowRight => {
                                    if self.cursor < BUFFER_SIZE {
                                        self.cursor += 1;
                                    }
                                    return String::new();
                                },
                                _ => {
                                    return String::new();
                                }
                            }
                        },

                        DecodedKey::Unicode(character) => {
                            println!("SHELL> Unicode character: {:?}", character);

                            match character {
                                '\u{8}' => { // backspace unicode
                                    println!("SHELL> Backspace from character");
                                    self.buffer[self.cursor] = '\0';
                                    if self.cursor > 0 {
                                        self.cursor -= 1;
                                    }
                                    return String::new();
                                },
                                '\n' => {
                                    let command = self.get_command();
                                    self.cursor = 0;
                                    self.start_last_command = 0;
                                    return self.execute(&command, String::new());
                                }
                                '\t' => {
                                    println!("SHELL> TAB From character");
                                    
                                    let input = self.get_command();
                                    let completion = self.complete(&input);
                                    for (i, c) in completion.chars().enumerate() {
                                        self.buffer[self.cursor + i] = c;
                                    }
                                    self.cursor += completion.len();
                                    self.cursor %= BUFFER_SIZE;
                                    return completion;
                                }
                                _ => {
                                    self.buffer[self.cursor] = character;
                                    self.cursor += 1;
                                    self.cursor %= BUFFER_SIZE;
                                    return String::new();
                                }
                            }
                        },
                    }
                }
            }
        }

        // never reached, for rust's error handling
        return String::new();
    }

    pub fn get_buffer(&self) -> String {
        self.buffer.iter().collect()
    }

    // Private commands

    fn command_help(&self) -> String {
        let mut output: String = "Available commands:\n".to_string();

        for command in self.commands.keys() {
            output.push_str(command);
            output.push_str("\n");
        }

        return output;
    }

    fn command_clear(&mut self) -> String {
        self.buffer = ['\0'; BUFFER_SIZE];
        self.cursor = 0;
        self.start_last_command = 0;
        "".to_string()
    }

    fn get_prompt(&self) -> String {
        format!("{} ", self.prefix)
    }

    fn get_command(&self) -> String {
        let command: String = self.buffer[self.start_last_command..self.cursor]
            .iter()
            .collect();
        println!("SHELL> Gotten Command: {}", command);
        format!("{}{}", self.get_prompt(), command)
    }
}

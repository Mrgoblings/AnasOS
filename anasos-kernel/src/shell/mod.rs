use alloc::{
    boxed::Box, collections::{btree_map::BTreeMap, vec_deque::VecDeque}, format, string::{String, ToString}, sync::Arc, vec::Vec
};
use crossbeam_queue::ArrayQueue;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

use crate::{apps::{terminal::BUFFER_SIZE, SCANCODE_QUEUE_SIZE}, println};

pub const STD_IN_SIZE: usize = 100; 

mod echo;
mod osfetch;

pub trait Command {
    fn execute(&self, args: Vec<&str>) -> String;
}


pub struct Buffer<const N: usize> {
    pub buffer: [char; N],
    pub cursor: usize,
}

impl<const N: usize> Buffer<N> {
    pub fn new() -> Self {
        Buffer {
            buffer: ['\0'; N], // Create a fixed-size array filled with '\0'
            cursor: 0,
        }
    }
}

/*
    ; Shell
    ; A simple shell that can execute commands
    ; Every shell has a prefix, which is the string that is printed before the user input
    ; The shell has a std_out of size BUFFER_SIZE, which is the maximum number of characters that can be typed
    ; The shell has a cursor_stdout, which is the current position in the std_out buffer
    ; The shell has a start_last_command, which is the position in the buffer where the last command started
    ; The shell has a BTreeMap of commands, which maps a command name to a Command trait object
    ;    The shell has 2 default commands: { help } and { clear }, which use the shell directly
    ;    The shell has a method to add custom commands
    ; The shell has an input queue, which is used to store scancodes
*/
pub struct Shell {
    std_out: Buffer<BUFFER_SIZE>,
    std_in: Buffer<STD_IN_SIZE>,

    prefix: &'static str,
    commands: BTreeMap<&'static str, Box<dyn Command>>,

    // input queue
    scancode_queue: Arc<ArrayQueue<u8>>,
    keyboard: Keyboard<layouts::Us104Key, ScancodeSet1>,

    // TODO change to not be a vec
    history: VecDeque<String>,
}

impl Shell {
    pub fn new(prefix: &'static str) -> Self {
        let mut commands: BTreeMap<&str, Box<dyn Command>> = BTreeMap::new();
        // set custom commands
        commands.insert("echo", Box::new(echo::Echo {}));
        commands.insert("osfetch", Box::new(osfetch::OsFetch {}));

        Shell {
            std_out: Buffer::<BUFFER_SIZE>::new(),
            std_in: Buffer::<STD_IN_SIZE>::new(),
            prefix,
            commands,
            scancode_queue: Arc::new(ArrayQueue::new(SCANCODE_QUEUE_SIZE)),
            keyboard: Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore,
            ),
            history: VecDeque::new(),
        }
    }

    fn add_char_to_std_out(&mut self, c: char) {
        self.std_out.buffer[self.std_out.cursor] = c;
        self.std_out.cursor += 1;
        self.std_out.cursor %= BUFFER_SIZE;
    }

    fn add_str_to_std_out(&mut self, s: &str) {
        for c in s.chars() {
            self.add_char_to_std_out(c);
        }
    }

    pub fn execute(&mut self, command_input: &str) {
        let command_trimmed = command_input.trim();
        println!("SHELL> Executing command: {}", command_input);

        self.add_str_to_std_out(format!("{}{}\n", self.prefix, command_input).as_str());

        if command_trimmed.is_empty() {
            println!("SHELL> Command is empty");
            return;
        }

        // NOT pushing to history if command starts with a space
        if !command_input.starts_with(" ") {
            self.history.push_back(command_trimmed.to_string());
        }

        let command_split: Vec<&str> = command_trimmed.split(" ").collect();

        let output;
        if command_split[0] == "help" {
            output = self.command_help();
        } else if command_split[0] == "clear" {
            output = self.command_clear();
        } else {
            match self.commands.get(command_split[0]) {
            Some(command) => output = command.execute(command_split[1..].to_vec()),
            None => output = format!("{}: command not found\n", command_split[0]),
            }
        }

        self.add_str_to_std_out(&output);
    }

    pub fn complete(&self, input: &str) -> Vec<String> {
        println!("SHELL> completions input: {}", input);

        println!("SHELL> completions: {:?}", self.commands.keys());

        let mut completions = self
            .commands
            .keys()
            .cloned()
            .chain(["clear", "help"].iter().cloned())// add default commands
            .filter(|command| command.starts_with(input))
            .map(|command| command.to_string())
            .collect::<Vec<String>>();

           completions.sort();

           println!("SHELL> completions: {:?}", completions);

           completions
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
    pub fn handle_input(&mut self) {
        //keyboard input
        while let Some(scancode) = self.scancode_queue.pop() {
            if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
                if let Some(key) = self.keyboard.process_keyevent(key_event) {
                    match key {
                        DecodedKey::RawKey(key_code) => {
                            match key_code {
                                pc_keyboard::KeyCode::Backspace | pc_keyboard::KeyCode::Tab => {
                                    // never reached, unicode version handles them
                                    return;
                                },
                                pc_keyboard::KeyCode::F1 | pc_keyboard::KeyCode::F2 | pc_keyboard::KeyCode::F3 | pc_keyboard::KeyCode::F4 | pc_keyboard::KeyCode::F5 | pc_keyboard::KeyCode::F6 | pc_keyboard::KeyCode::F7 | pc_keyboard::KeyCode::F8 | pc_keyboard::KeyCode::F9 | pc_keyboard::KeyCode::F10 | pc_keyboard::KeyCode::F11 | pc_keyboard::KeyCode::F12 => {
                                    return;
                                },
                                pc_keyboard::KeyCode::ArrowDown => {
                                    // TODO: handle empty lines from history. Needs to have only 1 empty line from the starting stdin.
                                    if self.history.len() == 0 {
                                        return;
                                    }
                                    self.history.push_back(self.get_stdin());
                                    let input = self.history.pop_front().expect("SHELL> history is empty");
                                    self.set_stdin(input);
                                    
                                    return;
                                },
                                pc_keyboard::KeyCode::ArrowUp => {
                                    // TODO: handle empty lines from history. Needs to have only 1 empty line from the starting stdin.
                                    if self.history.len() == 0 {
                                        return;
                                    }
                                    self.history.push_front(self.get_stdin());
                                    let input = self.history.pop_back().expect("SHELL> history is empty");
                                    self.set_stdin(input);

                                    return;
                                },
                                pc_keyboard::KeyCode::ArrowLeft => {
                                    if self.std_in.cursor > 0 {
                                        self.std_in.cursor -= 1;
                                    }
                                    return;
                                },
                                pc_keyboard::KeyCode::ArrowRight => {
                                    if self.std_in.cursor < BUFFER_SIZE {
                                        self.std_in.cursor += 1;
                                    }
                                    return;
                                },
                                _ => {
                                    return;
                                }
                            }
                        },

                        DecodedKey::Unicode(character) => {
                            match character {
                                '\u{8}' => { // backspace unicode
                                    if self.std_in.cursor > 0 {
                                        self.std_in.cursor -= 1;
                                    }
                                    return;
                                },
                                '\n' => {
                                    let command = self.get_stdin();
                                    self.std_in.cursor = 0;
                                    self.execute(&command);
                                    return;
                                }
                                '\t' => {
                                    
                                    let input = self.get_stdin();
                                    let completion = self.complete(&input);

                                    if completion.len() == 1 {
                                        self.set_stdin(format!("{} ", completion[0]));
                                        return;
                                    }
                                    // TODO: handle multiple completions
                                    return;
                                }
                                _ => {
                                    self.std_in.buffer[self.std_in.cursor] = character;
                                    self.std_in.cursor += 1;
                                    self.std_in.cursor %= BUFFER_SIZE;
                                    return;
                                }
                            }
                        },
                    }
                }
            }
        }

        // never reached, for rust's error handling
        return;
    }

    pub fn get_stdout(&self) -> String {
        self.std_out.buffer[0..self.std_out.cursor].iter().collect()
    }

    pub fn get_stdin(&self) -> String {
        self.std_in.buffer[0..self.std_in.cursor].iter().collect()
    }

    pub fn get_prompt(&self) -> String {
        format!("{} ", self.prefix)
    }

    pub fn get_printable(&self) -> String {
        format!("{}\n{}{}", self.get_stdout(), self.get_prompt(), self.get_stdin())
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
        self.std_out.cursor = 0;
        "".to_string()
    }

    fn set_stdin(&mut self, input: String) {
        for (i, c) in input.chars().enumerate() {
            self.std_in.buffer[i] = c;
        }
        self.std_in.cursor = input.len();
    }
}

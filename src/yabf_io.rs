use super::command::Command;
use std::str::FromStr;
pub struct YabfIO {
    pub current_code: String,
    pub command_queue: Vec<Command>,
    pub is_typing_code: bool,
    pub command_history: Vec<Command>,
}

impl Default for YabfIO {
    fn default() -> Self {
        Self {
            current_code: String::new(),
            command_queue: vec![],
            is_typing_code: false,
            command_history: vec![],
        }
    }
}

impl YabfIO {
    pub fn try_read_command(&mut self) {
        use std::io::stdin;
        let mut input_buf = String::new();

        let mut keep_typing = true;
        let mut line_count = 1;

        while keep_typing {
            if self.is_typing_code {
                print!("{line_count} ");
                use std::io::{stdout, Write};
                stdout().flush().expect("Couldn't flush stdout.");
                line_count += 1;
            }
            keep_typing = self.is_typing_code;
            input_buf.clear();
            match stdin().read_line(&mut input_buf) {
                Err(_) => break,
                Ok(_) => {
                    if let Some('\n') = input_buf.chars().next_back() {
                        input_buf.pop();
                    }
                    if let Some('\r') = input_buf.chars().next_back() {
                        input_buf.pop();
                    }
                }
            }
            if let Ok(c) = Command::from_str(&input_buf) {
                match c {
                    Command::End => {
                        self.command_queue.push(c);
                        break;
                    }
                    _ => self.command_queue.push(c),
                }
            }

            if self.is_typing_code {
                self.current_code += &input_buf;
                self.current_code.push('\n');
            }
        }
    }
}

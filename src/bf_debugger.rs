use yabf::{BfInstance, Program};

use crate::{command::Command, yabf_io::YabfIO};

const HELP_TEXT: &str = r#"
Commands:
    HELP: Show this text.
    EXIT: Exit yabf-cli.
    RUN: Run the currently selected program.
    BEGIN: Start writing code to the code buffer.
    END: Stop writing code to the code buffer.
    CLEAR: Clear the code buffer.
    SHOW: Print the code buffer.
    SET: Parse the code buffer and set it as the currently selected program.
"#;

pub enum BfDebugControlFlow {
    Exit,
    Run,
}

pub struct BfDebugConfig {
    pub dump_padding: usize,
}

impl Default for BfDebugConfig {
    fn default() -> Self {
        Self { dump_padding: 5 }
    }
}

pub struct BfDebugger<const MEMSIZE: usize> {
    pub bf: BfInstance<MEMSIZE>,
    pub cfg: BfDebugConfig,
    pub io: YabfIO,
}

impl<T: AsRef<str>, const MEMSIZE: usize> From<T> for BfDebugger<MEMSIZE> {
    fn from(s: T) -> Self {
        let c = s.as_ref();
        let mut res = Self::default();
        res.io.current_code = c.to_string();
        res.io.current_code.push('\n');
        let p = Program::from(s);
        res.bf.program = p;

        res
    }
}

impl<const MEMSIZE: usize> Default for BfDebugger<MEMSIZE> {
    fn default() -> Self {
        Self {
            bf: Default::default(),
            cfg: Default::default(),
            io: Default::default(),
        }
    }
}

impl<const MEMSIZE: usize> BfDebugger<MEMSIZE> {
    pub fn step_command(&mut self) -> BfDebugControlFlow {
        if self.io.command_queue.len() < 1 {
            return BfDebugControlFlow::Run;
        }
        match self.io.command_queue.pop().unwrap() {
            Command::Help => {
                println!("{}", HELP_TEXT);
                BfDebugControlFlow::Run
            }
            Command::Begin => {
                self.io.is_typing_code = true;
                BfDebugControlFlow::Run
            }
            Command::End => {
                self.io.is_typing_code = false;
                BfDebugControlFlow::Run
            }
            Command::Clear => {
                self.io.current_code.clear();
                BfDebugControlFlow::Run
            }
            Command::Run => {
                self.bf.run();
                println!();
                self.bf.mem = [0; MEMSIZE];
                self.bf.mem_ptr = 0;
                self.bf.program.counter = 0;
                BfDebugControlFlow::Run
            }
            Command::Exit => BfDebugControlFlow::Exit,
            Command::Show => {
                println!();
                for (i, l) in self.io.current_code.lines().enumerate() {
                    println!("{} {l}", i + 1);
                }
                println!();
                BfDebugControlFlow::Run
            }
            Command::Set => {
                let p = Program::from(self.io.current_code.clone());
                self.bf.program = p;
                BfDebugControlFlow::Run
            }
        }
    }
    pub fn dump_mem(&self, slice_len: usize) -> String {
        if slice_len >= MEMSIZE {
            panic!("Slice length can't be bigger than memory size.")
        }
        if self.cfg.dump_padding < 5 {
            panic!("Dump padding can't be less than 5.")
        }
        // Wrote this on my phone so it's a bit scuffed lol
        let padding = self.cfg.dump_padding;
        let start: usize = match self.bf.mem_ptr <= slice_len {
            true => 0,
            false => self.bf.mem_ptr - slice_len / 2,
        };
        let end: usize = match start == 0 {
            true => slice_len,
            false => {
                self.bf.mem_ptr
                    + slice_len / 2
                    + match slice_len % 2 == 0 {
                        false => 1,
                        true => 0,
                    }
            }
        };
        let mut res = " ".to_string();
        for _ in 0..(slice_len - 1) * (padding + 1) + padding {
            res += "_";
        }
        res += "\n|";

        for i in start..end {
            let n = &self.bf.mem[i];
            if i == self.bf.mem_ptr {
                let p = padding - 2;
                res += format!("[{:^p$}]|", n).as_str();
            } else {
                res += format!("{:^padding$}|", n).as_str();
            }
        }
        res += "\n ";
        for _ in 0..(slice_len - 1) * (padding + 1) + padding {
            res += "-";
        }
        res
    }

    pub fn prompt(&mut self) {
        if !self.io.is_typing_code {
            print!(">>");
            {
                use std::io::{stdout, Write};
                stdout().flush().expect("Couldn't flush stdout.");
            }
        }
        self.io.try_read_command();
    }
}

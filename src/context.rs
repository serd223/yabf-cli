use yabf::{BfInstance, Instruction, Program, ProgramStatus};

use crate::{command::Command, io::IO};

pub const HELP_TEXT: &str = r#"
Commands:
    HELP: Show this text.
    EXIT: Exit yabf-cli.
    RUN: Run the currently selected program.
    BEGIN: Start writing code to the code buffer.
    END: Stop writing code to the code buffer.
    CLEAR: Clear the code buffer.
    SHOW: Print the code buffer.
    SET: Parse the code buffer and set it as the currently selected program.
    DBG/DEBUG: Start debugging the current program.

Debug Mode Commands:
    N/NEXT: Step to the next instruction.
    NO/NEXTOUT: Keep stepping until the next Out instruction and print the program output.
    D/DMP/DUMP: Visualize the current program memory.
    O/OUT: Show current program out buffer.
    ED/ENDDEBUG: Stop debugging the current program and reset program memory.
"#;

pub enum ControlFlow {
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

pub struct Context<const MEMSIZE: usize> {
    pub bf: BfInstance<MEMSIZE>,
    pub cfg: BfDebugConfig,
    pub io: IO,
    debug_mode: bool,
}

impl<T: AsRef<str>, const MEMSIZE: usize> From<T> for Context<MEMSIZE> {
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

impl<const MEMSIZE: usize> Default for Context<MEMSIZE> {
    fn default() -> Self {
        Self {
            bf: Default::default(),
            cfg: Default::default(),
            io: Default::default(),
            debug_mode: false,
        }
    }
}

impl<const MEMSIZE: usize> Context<MEMSIZE> {
    pub fn step_command(&mut self) -> ControlFlow {
        if self.io.command_queue.len() < 1 {
            return ControlFlow::Run;
        }
        match self.io.command_queue.pop().unwrap() {
            Command::Help => {
                println!("{}", HELP_TEXT);
                ControlFlow::Run
            }
            Command::Begin => {
                self.io.is_typing_code = true;
                ControlFlow::Run
            }
            Command::End => {
                self.io.is_typing_code = false;
                ControlFlow::Run
            }
            Command::Clear => {
                self.io.current_code.clear();
                ControlFlow::Run
            }
            Command::Run => {
                self.bf.run();
                println!();
                self.bf.mem = [0; MEMSIZE];
                self.bf.mem_ptr = 0;
                self.bf.program.counter = 0;
                ControlFlow::Run
            }
            Command::Exit => ControlFlow::Exit,
            Command::Show => {
                println!();
                for (i, l) in self.io.current_code.lines().enumerate() {
                    println!("{} {l}", i + 1);
                }
                println!();
                ControlFlow::Run
            }
            Command::Set => {
                let p = Program::from(self.io.current_code.clone());
                self.bf.program = p;
                ControlFlow::Run
            }

            Command::Debug => {
                self.debug_mode = true;
                ControlFlow::Run
            }
            Command::Next => {
                if self.debug_mode {
                    match self.bf.step() {
                        ProgramStatus::Exit => {
                            self.debug_mode = false;
                            println!("The Program has ended.\nProgram Output:");
                            while let Some(c) = self.bf.io_buf.pop_out() {
                                print!("{c}")
                            }
                            println!();
                            self.bf.mem = [0; MEMSIZE];
                            self.bf.mem_ptr = 0;
                            self.bf.program.counter = 0;
                        }
                        _ => {
                            println!(
                                "Current Instruction: {}; Program Counter: {}",
                                match self.bf.program.current() {
                                    Instruction::Add => '+',
                                    Instruction::Sub => '-',
                                    Instruction::Left => '<',
                                    Instruction::Right => '>',
                                    Instruction::LoopStart(_) => '[',
                                    Instruction::LoopEnd(_) => ']',
                                    Instruction::Out => ',',
                                    Instruction::In => ',',
                                },
                                self.bf.program.counter
                            );
                        }
                    }
                }
                ControlFlow::Run
            }
            Command::NextOut => {
                if self.debug_mode {
                    loop {
                        match self.bf.program.current() {
                            Instruction::Out => break,
                            _ => (),
                        }
                        match self.bf.step() {
                            ProgramStatus::Exit => {
                                self.debug_mode = false;
                                println!("The Program has ended.\nProgram Output:");
                                while let Some(c) = self.bf.io_buf.pop_out() {
                                    print!("{c}")
                                }
                                println!();
                                self.bf.mem = [0; MEMSIZE];
                                self.bf.mem_ptr = 0;
                                self.bf.program.counter = 0;
                                break;
                            }
                            _ => (),
                        }
                    }
                    self.io.command_queue.push(Command::Out);
                    self.io.command_queue.push(Command::Next);
                    self.step_command();
                    self.step_command();
                }
                ControlFlow::Run
            }
            Command::Dump => {
                if self.debug_mode {
                    println!("{}", self.dump_mem(7));
                }
                ControlFlow::Run
            }
            Command::Out => {
                if self.debug_mode {
                    let mut o = self.bf.io_buf.out_buf.clone();
                    while let Some(c) = o.pop() {
                        print!("{c}")
                    }
                    println!();
                }
                ControlFlow::Run
            }
            Command::EndDebug => {
                self.debug_mode = false;
                self.bf.mem = [0; MEMSIZE];
                self.bf.mem_ptr = 0;
                self.bf.program.counter = 0;
                ControlFlow::Run
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

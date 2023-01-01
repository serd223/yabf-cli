use context::*;

mod command;
mod context;
mod io;

const DEFAULT_PROGRAM: &str = r#"
>>++++++[-<++++>]<+[-<+++>]<------.  
"#;

fn main() {
    println!("Welcome to yabf-cli!");
    println!("{HELP_TEXT}");
    let mut bf_dbg: Context<256> = Context::from(DEFAULT_PROGRAM);
    bf_dbg.prompt();
    loop {
        let r = bf_dbg.step_command();
        match r {
            ControlFlow::Exit => break,
            _ => (),
        }
        bf_dbg.prompt();
    }
}

use context::*;

mod command;
mod context;
mod io;

fn main() {
    let mut bf_dbg: Context<256> = Context::from(">+++++++++[<++++>-]<.");
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

use bf_debugger::*;

mod bf_debugger;
mod command;
mod yabf_io;

fn main() {
    let mut bf_dbg: BfDebugger<256> = BfDebugger::from(">+++++++++[<++++>-]<.");
    bf_dbg.prompt();
    loop {
        let r = bf_dbg.step_command();
        match r {
            BfDebugControlFlow::Exit => break,
            _ => (),
        }
        bf_dbg.prompt();
    }
}

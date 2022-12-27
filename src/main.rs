use bf_debugger::*;
use yabf::{BfInstance, Program};

mod bf_debugger;
mod command;
mod yabf_io;

fn main() {
    let program = Program::from(">+++++++++[<++++>-]<.");
    let bf: BfInstance<256> = BfInstance::from(program);
    let mut bf_dbg: BfDebugger<256> = BfDebugger::from(bf);
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

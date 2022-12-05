use bf_debugger::BfDebugger;
use yabf::{Program, BfInstance};

mod bf_debugger;

fn main() {
    // Temporary debug
    let program = Program::from(">+++++++++++[<+++++++++++>-]<>>++++[<++++>-]<>++++++<<");
    let program2 = Program::from(">>>>>>>>>>>>>>>>>>>>>>>>+++++++++++[<+++++++++++>-]<>>++++[<++++>-]<>++++++<<");
    
    let mut bf: BfInstance<256> = BfInstance::from(program);
    bf.run();
    
    let mut bf2: BfInstance<256> = BfInstance::from(program2);
    bf2.run();
    
    let mut debugger = BfDebugger {
        bf: bf,
        ..Default::default()
    };
    let mut debugger2 = BfDebugger {
        bf: bf2,
        ..Default::default()
    };
    for p in 5..14 {
        debugger.cfg.dump_padding = p;
        debugger2.cfg.dump_padding = p;
        println!("Padding: {p}\n");
        println!("Len: 11");
        println!("{}", debugger.dump_mem(11));
        println!("{}", debugger2.dump_mem(11));
        
        println!("Len: 10");
        println!("{}", debugger.dump_mem(10));
        println!("{}", debugger2.dump_mem(10));
        println!("{}", (0..100).map(|_| '#').collect::<String>());
    }
}
